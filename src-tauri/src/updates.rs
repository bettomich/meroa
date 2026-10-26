use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::Notify;

const RELEASE_API: &str = "https://api.github.com/repos/bettomich/meroa/releases/latest";
const RELEASE_ROOT: &str = "https://github.com/bettomich/meroa/releases";
const WEEK: u64 = 7 * 24 * 60 * 60;
const RETRY_DELAY: u64 = 24 * 60 * 60;
const MANUAL_COOLDOWN: u64 = 60;
const MAX_METADATA: usize = 256 * 1024;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableUpdate {
    pub version: String,
    pub download_url: String,
    pub release_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct UpdateState {
    schema: u8,
    pub current_version: String,
    pub automatic_checks: bool,
    pub checked_at: Option<u64>,
    pub last_attempt_at: Option<u64>,
    pub next_check_at: u64,
    pub available: Option<AvailableUpdate>,
    pub checking: bool,
    pub error: bool,
}

impl Default for UpdateState {
    fn default() -> Self {
        Self {
            schema: 1,
            current_version: env!("CARGO_PKG_VERSION").into(),
            automatic_checks: true,
            checked_at: None,
            last_attempt_at: None,
            next_check_at: 0,
            available: None,
            checking: false,
            error: false,
        }
    }
}

#[derive(Default)]
pub struct UpdateStore {
    state: Mutex<UpdateState>,
    path: Mutex<Option<PathBuf>>,
    wake: Notify,
}

#[derive(Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
    state: String,
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    draft: bool,
    prerelease: bool,
    assets: Vec<ReleaseAsset>,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn release_links(version: &str) -> Option<AvailableUpdate> {
    let parsed = semver::Version::parse(version).ok()?;
    if !parsed.pre.is_empty() || !parsed.build.is_empty() || parsed.to_string() != version {
        return None;
    }
    Some(AvailableUpdate {
        version: version.into(),
        release_url: format!("{RELEASE_ROOT}/tag/v{version}"),
        download_url: format!("{RELEASE_ROOT}/download/v{version}/MEROA_{version}_x64-setup.exe"),
    })
}

fn parse_release(bytes: &[u8], current: &str) -> Result<Option<AvailableUpdate>, ()> {
    let release: Release = serde_json::from_slice(bytes).map_err(|_| ())?;
    if release.draft || release.prerelease {
        return Ok(None);
    }
    let version = release.tag_name.strip_prefix('v').ok_or(())?;
    let links = release_links(version).ok_or(())?;
    let latest = semver::Version::parse(version).map_err(|_| ())?;
    if latest <= semver::Version::parse(current).map_err(|_| ())? {
        return Ok(None);
    }
    let expected_name = format!("MEROA_{version}_x64-setup.exe");
    if !release.assets.iter().any(|asset| {
        asset.name == expected_name
            && asset.browser_download_url == links.download_url
            && asset.state == "uploaded"
    }) {
        return Err(());
    }
    Ok(Some(links))
}

fn normalized_cache(mut state: UpdateState, timestamp: u64) -> UpdateState {
    if state.schema != 1 {
        return UpdateState::default();
    }
    state.current_version = env!("CARGO_PKG_VERSION").into();
    state.checking = false;
    state.error = false;
    // A damaged cache or a clock adjustment must not suppress checks indefinitely.
    if state.next_check_at > timestamp.saturating_add(WEEK) {
        state.next_check_at = 0;
    }
    if state.last_attempt_at.is_some_and(|last| last > timestamp) {
        state.last_attempt_at = None;
    }
    if state.checked_at.is_some_and(|last| last > timestamp) {
        state.checked_at = None;
    }
    state.available = state.available.filter(|update| {
        release_links(&update.version).as_ref() == Some(update)
            && semver::Version::parse(&update.version).ok()
                > semver::Version::parse(&state.current_version).ok()
    });
    state
}

fn may_check(state: &UpdateState, manual: bool, timestamp: u64) -> bool {
    if state.checking {
        return false;
    }
    if manual {
        return state.last_attempt_at.is_none_or(|last| {
            timestamp < last || timestamp.saturating_sub(last) >= MANUAL_COOLDOWN
        });
    }
    state.automatic_checks && timestamp >= state.next_check_at
}

fn persist(store: &UpdateStore, state: &UpdateState) {
    let path = store.path.lock().ok().and_then(|path| path.clone());
    if let Some(path) = path {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(bytes) = serde_json::to_vec(state) {
            let _ = fs::write(path, bytes);
        }
    }
}

fn emit(app: &AppHandle, state: &UpdateState) {
    let _ = app.emit("update-state", state);
}

#[tauri::command]
pub fn get_update_state(app: AppHandle) -> UpdateState {
    app.state::<UpdateStore>()
        .state
        .lock()
        .map(|state| state.clone())
        .unwrap_or_default()
}

#[tauri::command]
pub fn set_automatic_update_checks(app: AppHandle, enabled: bool) -> Result<UpdateState, String> {
    let store = app.state::<UpdateStore>();
    let mut state = store.state.lock().map_err(|_| "update state unavailable")?;
    state.automatic_checks = enabled;
    persist(&store, &state);
    emit(&app, &state);
    store.wake.notify_one();
    Ok(state.clone())
}

async fn fetch_release(current: &str) -> Result<Option<AvailableUpdate>, ()> {
    // Allocate networking resources only when a check is actually due.
    let client = reqwest::Client::builder()
        .user_agent(concat!("MEROA/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(12))
        .connect_timeout(Duration::from_secs(5))
        .redirect(reqwest::redirect::Policy::none())
        .https_only(true)
        .build()
        .map_err(|_| ())?;
    let mut response = client
        .get(RELEASE_API)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await
        .map_err(|_| ())?
        .error_for_status()
        .map_err(|_| ())?;
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| ())? {
        if bytes.len() + chunk.len() > MAX_METADATA {
            return Err(());
        }
        bytes.extend_from_slice(&chunk);
    }
    parse_release(&bytes, current)
}

fn finish_check(
    state: &mut UpdateState,
    result: Result<Option<AvailableUpdate>, ()>,
    timestamp: u64,
) {
    state.checking = false;
    match result {
        Ok(available) => {
            state.available = available;
            state.error = false;
            state.checked_at = Some(timestamp);
            state.next_check_at = timestamp.saturating_add(WEEK);
        }
        Err(_) => {
            state.error = true;
            state.next_check_at = timestamp.saturating_add(RETRY_DELAY);
        }
    }
}

async fn check(app: &AppHandle, manual: bool) -> UpdateState {
    let store = app.state::<UpdateStore>();
    let current = {
        let Ok(mut state) = store.state.lock() else {
            return UpdateState::default();
        };
        if !may_check(&state, manual, now()) {
            return state.clone();
        }
        state.checking = true;
        state.error = false;
        state.last_attempt_at = Some(now());
        // Record the attempt before the request, preventing repeated offline
        // requests across app restarts. A failed request never erases an update.
        state.next_check_at = now().saturating_add(RETRY_DELAY);
        persist(&store, &state);
        emit(app, &state);
        state.current_version.clone()
    };
    let result = fetch_release(&current).await;
    let Ok(mut state) = store.state.lock() else {
        return UpdateState::default();
    };
    finish_check(&mut state, result, now());
    persist(&store, &state);
    emit(app, &state);
    store.wake.notify_one();
    state.clone()
}

#[tauri::command]
pub async fn check_for_update(app: AppHandle) -> UpdateState {
    check(&app, true).await
}

#[tauri::command]
pub fn open_update_download(app: AppHandle, version: String) -> Result<(), String> {
    let state = get_update_state(app);
    let update = state
        .available
        .filter(|update| update.version == version)
        .and_then(|update| release_links(&update.version))
        .ok_or("update unavailable")?;
    // Only a validated, canonical URL for this repository can reach the OS.
    // This opens the user's browser; it never runs a downloaded executable.
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let windows = std::env::var_os("SystemRoot").ok_or("Windows directory unavailable")?;
        std::process::Command::new(PathBuf::from(windows).join("System32/rundll32.exe"))
            .arg("url.dll,FileProtocolHandler")
            .arg(update.download_url)
            .creation_flags(0x08000000)
            .spawn()
            .map_err(|_| "could not open download")?;
        Ok(())
    }
    #[cfg(not(windows))]
    {
        let _ = update;
        Err("Windows download is only supported on Windows".into())
    }
}

pub fn start(app: AppHandle) {
    let store = app.state::<UpdateStore>();
    if let Ok(directory) = app.path().app_local_data_dir() {
        let path = directory.join("update-check.v1.json");
        let cached = fs::metadata(&path)
            .ok()
            .filter(|metadata| metadata.len() <= 16 * 1024)
            .and_then(|_| fs::read(&path).ok())
            .and_then(|bytes| serde_json::from_slice::<UpdateState>(&bytes).ok())
            .map(|state| normalized_cache(state, now()))
            .unwrap_or_default();
        if let Ok(mut state) = store.state.lock() {
            *state = cached;
        }
        if let Ok(mut stored_path) = store.path.lock() {
            *stored_path = Some(path);
        }
    }
    tauri::async_runtime::spawn(async move {
        // Let startup and the first usage read complete before optional networking.
        tokio::time::sleep(Duration::from_secs(20)).await;
        loop {
            let store = app.state::<UpdateStore>();
            let state = match store.state.lock() {
                Ok(state) => state.clone(),
                Err(_) => break,
            };
            if !state.automatic_checks {
                store.wake.notified().await;
            } else if state.checking {
                store.wake.notified().await;
            } else if now() >= state.next_check_at {
                check(&app, false).await;
            } else {
                let delay = Duration::from_secs(state.next_check_at.saturating_sub(now()).max(1));
                let _ = tokio::time::timeout(delay, store.wake.notified()).await;
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata(version: &str) -> serde_json::Value {
        serde_json::json!({"tag_name":format!("v{version}"),"draft":false,"prerelease":false,"assets":[{
            "name":format!("MEROA_{version}_x64-setup.exe"),"state":"uploaded",
            "browser_download_url":format!("{RELEASE_ROOT}/download/v{version}/MEROA_{version}_x64-setup.exe")
        }]})
    }

    #[test]
    fn checks_weekly_with_restart_caching_and_manual_cooldown() {
        let mut state = UpdateState::default();
        assert!(may_check(&state, false, 100));
        state.next_check_at = 100 + WEEK;
        state.last_attempt_at = Some(100);
        assert!(!may_check(&state, false, 100 + WEEK - 1));
        assert!(may_check(&state, false, 100 + WEEK));
        assert!(!may_check(&state, true, 159));
        assert!(may_check(&state, true, 160));
        assert!(may_check(&state, true, 90));
        let cached: UpdateState =
            serde_json::from_slice(&serde_json::to_vec(&state).unwrap()).unwrap();
        assert!(!may_check(&normalized_cache(cached, 101), false, 101));
        state.automatic_checks = false;
        assert!(!may_check(&state, false, 100 + WEEK));
        assert!(may_check(&state, true, 100 + WEEK));
        state.checking = true;
        assert!(!may_check(&state, true, 100 + WEEK));
    }

    #[test]
    fn versions_compare_numerically_and_only_offer_newer_stable_installers() {
        let parse = |value: serde_json::Value, current| {
            parse_release(&serde_json::to_vec(&value).unwrap(), current)
        };
        assert_eq!(
            parse(metadata("0.10.0"), "0.9.0").unwrap().unwrap().version,
            "0.10.0"
        );
        assert!(parse(metadata("0.2.0"), "0.2.0").unwrap().is_none());
        assert!(parse(metadata("0.1.0"), "0.2.0").unwrap().is_none());
        let mut value = metadata("0.3.0");
        value["prerelease"] = true.into();
        assert!(parse(value, "0.2.0").unwrap().is_none());
        let mut value = metadata("0.3.0");
        value["draft"] = true.into();
        assert!(parse(value, "0.2.0").unwrap().is_none());
        let mut value = metadata("0.3.0");
        value["assets"] = serde_json::json!([]);
        assert!(parse(value, "0.2.0").is_err());
        let mut value = metadata("0.3.0");
        value["assets"][0]["browser_download_url"] = "https://example.com/installer.exe".into();
        assert!(parse(value, "0.2.0").is_err());
        assert!(parse(metadata("0.3.0-beta.1"), "0.2.0").is_err());
        assert!(release_links("0.3.0&calc.exe").is_none());
    }

    #[test]
    fn cache_drops_installed_or_tampered_updates_and_recovers_from_clock_changes() {
        let mut state = UpdateState::default();
        state.available = release_links(&state.current_version);
        state.checking = true;
        state.next_check_at = u64::MAX;
        state.last_attempt_at = Some(u64::MAX);
        state.checked_at = Some(u64::MAX);
        let normalized = normalized_cache(state, 100);
        assert!(normalized.available.is_none());
        assert!(!normalized.checking);
        assert_eq!(normalized.next_check_at, 0);
        assert!(normalized.last_attempt_at.is_none());
        let mut state = UpdateState::default();
        let mut update = release_links("99.0.0").unwrap();
        update.download_url = "https://example.com/install.exe".into();
        state.available = Some(update);
        assert!(normalized_cache(state, 100).available.is_none());
        assert!(serde_json::from_slice::<UpdateState>(b"broken cache").is_err());
    }

    #[test]
    fn offline_failures_retain_the_offer_and_back_off_without_changing_last_success() {
        let mut state = UpdateState::default();
        state.available = release_links("99.0.0");
        state.checked_at = Some(100);
        state.checking = true;
        finish_check(&mut state, Err(()), 200);
        assert_eq!(state.available.as_ref().unwrap().version, "99.0.0");
        assert_eq!(state.checked_at, Some(100));
        assert!(!state.checking);
        assert!(state.error);
        assert!(!may_check(&state, false, 200 + RETRY_DELAY - 1));
        assert!(may_check(&state, false, 200 + RETRY_DELAY));
        finish_check(&mut state, Ok(None), 300);
        assert!(state.available.is_none());
        assert!(!state.error);
        assert_eq!(state.checked_at, Some(300));
        assert_eq!(state.next_check_at, 300 + WEEK);
    }

    #[test]
    #[ignore = "read-only HTTPS smoke test against the public GitHub release endpoint"]
    fn live_github_metadata_uses_native_tls_and_matches_the_official_installer() {
        let update = tauri::async_runtime::block_on(fetch_release("0.0.0"))
            .expect("GitHub release metadata should load over HTTPS")
            .expect("the repository should contain a stable Windows release");
        assert_eq!(Some(update.clone()), release_links(&update.version));
    }
}
