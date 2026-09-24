use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    env, fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{mpsc, Mutex, OnceLock},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const FIVE_HOURS_MINUTES: u64 = 5 * 60;
const WEEKLY_MINUTES: u64 = 7 * 24 * 60;
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(20);
const MAX_JSONL_LINE_BYTES: usize = 32 * 1024;
const MAX_RESPONSE_BYTES: usize = 128 * 1024;
const MAX_CREDIT_BALANCE_BYTES: usize = 16;
const MAX_RESET_TIMESTAMP: i64 = 4_102_444_800; // 2100-01-01 UTC

static ACTIVE_CHILD: OnceLock<Mutex<Option<Child>>> = OnceLock::new();

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WindowKind {
    FiveHours,
    Weekly,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageWindow {
    pub kind: WindowKind,
    pub used_percent: f64,
    pub remaining_percent: u8,
    pub resets_at: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Credits {
    pub has_credits: bool,
    pub unlimited: bool,
    pub balance: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitingWindow {
    pub kind: WindowKind,
    pub remaining_percent: u8,
    pub resets_at: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexUsage {
    pub five_hours: Option<UsageWindow>,
    pub weekly: Option<UsageWindow>,
    pub credits: Option<Credits>,
    pub limiting_window: LimitingWindow,
    pub fetched_at: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FetchError {
    Unavailable,
    SignInRequired,
    Timeout,
    InvalidResponse,
    Source,
}

impl FetchError {
    pub fn message(&self) -> &str {
        match self {
            Self::Unavailable => "Codex usage is unavailable.",
            Self::SignInRequired => "Sign in to Codex with ChatGPT to make usage data available.",
            Self::Timeout => "Codex usage request timed out.",
            Self::InvalidResponse => "Codex returned invalid usage data.",
            Self::Source => "Could not read Codex usage.",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RateLimitsResponse {
    rate_limits: Option<RateLimitSnapshot>,
    #[serde(default)]
    rate_limits_by_limit_id: HashMap<String, RateLimitSnapshot>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RateLimitSnapshot {
    primary: Option<SourceWindow>,
    secondary: Option<SourceWindow>,
    credits: Option<SourceCredits>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SourceWindow {
    used_percent: f64,
    window_duration_mins: Option<u64>,
    resets_at: Option<i64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SourceCredits {
    has_credits: bool,
    unlimited: bool,
    balance: String,
}

fn remaining_percent(used_percent: f64) -> u8 {
    let used_percent = if used_percent.is_finite() {
        used_percent.clamp(0.0, 100.0)
    } else {
        0.0
    };
    (100.0 - used_percent).round() as u8
}

fn normalize_window(source: &SourceWindow, kind: WindowKind) -> UsageWindow {
    UsageWindow {
        kind,
        used_percent: if source.used_percent.is_finite() {
            source.used_percent.clamp(0.0, 100.0)
        } else {
            0.0
        },
        remaining_percent: remaining_percent(source.used_percent),
        resets_at: source
            .resets_at
            .filter(|timestamp| (0..=MAX_RESET_TIMESTAMP).contains(timestamp)),
    }
}

fn normalize_response(
    response: RateLimitsResponse,
    fetched_at: i64,
) -> Result<CodexUsage, FetchError> {
    let snapshot = response
        .rate_limits_by_limit_id
        .get("codex")
        .cloned()
        .or(response.rate_limits)
        .ok_or(FetchError::Unavailable)?;

    let windows = [snapshot.primary.as_ref(), snapshot.secondary.as_ref()];
    let five_hours = windows
        .iter()
        .flatten()
        .find(|window| window.window_duration_mins == Some(FIVE_HOURS_MINUTES))
        .map(|window| normalize_window(window, WindowKind::FiveHours));
    let weekly = windows
        .iter()
        .flatten()
        .find(|window| window.window_duration_mins == Some(WEEKLY_MINUTES))
        .map(|window| normalize_window(window, WindowKind::Weekly));

    let limiting = match (&five_hours, &weekly) {
        (Some(five_hours), Some(weekly)) => {
            if five_hours.remaining_percent <= weekly.remaining_percent {
                five_hours
            } else {
                weekly
            }
        }
        (Some(five_hours), None) => five_hours,
        (None, Some(weekly)) => weekly,
        (None, None) => return Err(FetchError::InvalidResponse),
    };
    let limiting_window = LimitingWindow {
        kind: limiting.kind,
        remaining_percent: limiting.remaining_percent,
        resets_at: limiting.resets_at,
    };

    let credits = snapshot.credits.map(|credits| {
        let balance = credits.balance.trim();
        let balance = if credits.unlimited {
            "∞".to_string()
        } else if !balance.is_empty()
            && balance.len() <= MAX_CREDIT_BALANCE_BYTES
            && balance.bytes().all(|byte| byte.is_ascii_digit())
        {
            balance.to_string()
        } else {
            "—".to_string()
        };
        Credits {
            has_credits: credits.has_credits,
            unlimited: credits.unlimited,
            balance,
        }
    });

    Ok(CodexUsage {
        five_hours,
        weekly,
        credits,
        limiting_window,
        fetched_at,
    })
}

fn trusted_executable(bin_root: &Path, candidate: PathBuf) -> Option<PathBuf> {
    let root = bin_root.canonicalize().ok()?;
    let candidate = candidate.canonicalize().ok()?;
    (candidate.is_file() && candidate.starts_with(root)).then_some(candidate)
}

fn executable_from_bin_root(bin_root: &Path) -> Option<PathBuf> {
    let direct = trusted_executable(bin_root, bin_root.join("codex.exe"));
    if direct.is_some() {
        return direct;
    }

    let candidates = fs::read_dir(bin_root)
        .ok()?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            entry
                .file_type()
                .ok()
                .filter(|kind| kind.is_dir())
                .map(|_| entry)
        })
        .filter_map(|entry| trusted_executable(bin_root, entry.path().join("codex.exe")))
        .filter_map(|path| {
            fs::metadata(&path)
                .ok()?
                .modified()
                .ok()
                .map(|modified| (path, modified))
        })
        .collect::<Vec<_>>();
    let newest = candidates.iter().map(|(_, modified)| *modified).max()?;
    let mut newest_candidates = candidates
        .into_iter()
        .filter_map(|(path, modified)| (modified == newest).then_some(path));
    let candidate = newest_candidates.next()?;
    newest_candidates.next().is_none().then_some(candidate)
}

fn codex_executable() -> Result<PathBuf, FetchError> {
    let local_app_data = env::var_os("LOCALAPPDATA").ok_or(FetchError::Unavailable)?;
    let bin_root = PathBuf::from(local_app_data)
        .join("OpenAI")
        .join("Codex")
        .join("bin");
    // Deliberately no PATH fallback: MEROA only starts an absolute executable from the
    // known Codex installation tree, and fails closed if that tree is ambiguous.
    executable_from_bin_root(&bin_root).ok_or(FetchError::Unavailable)
}

fn active_child() -> &'static Mutex<Option<Child>> {
    ACTIVE_CHILD.get_or_init(|| Mutex::new(None))
}

fn spawn_app_server() -> Result<(), FetchError> {
    let executable = codex_executable()?;
    let mut command = Command::new(executable);
    command
        .arg("app-server")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let child = command.spawn().map_err(|_| FetchError::Source)?;
    let mut active = active_child().lock().map_err(|_| FetchError::Source)?;
    if active.is_some() {
        let mut child = child;
        let _ = child.kill();
        let _ = child.wait();
        return Err(FetchError::Source);
    }
    *active = Some(child);
    Ok(())
}

fn send_request() -> Result<(), FetchError> {
    let mut active = active_child().lock().map_err(|_| FetchError::Source)?;
    let stdin = active
        .as_mut()
        .ok_or(FetchError::Source)?
        .stdin
        .as_mut()
        .ok_or(FetchError::Source)?;
    for message in [
        r#"{"method":"initialize","id":0,"params":{"clientInfo":{"name":"meroa","title":"MEROA","version":"0.1.0"}}}"#,
        r#"{"method":"initialized","params":{}}"#,
        r#"{"method":"account/rateLimits/read","id":1}"#,
    ] {
        writeln!(stdin, "{message}").map_err(|_| FetchError::Source)?;
    }
    stdin.flush().map_err(|_| FetchError::Source)
}

fn response_from_line(line: &[u8]) -> Result<Option<Value>, FetchError> {
    let line = line.strip_suffix(b"\r").unwrap_or(line);
    if line.is_empty() {
        return Ok(None);
    }
    let value = serde_json::from_slice::<Value>(line).map_err(|_| FetchError::InvalidResponse)?;
    Ok((value.get("id").and_then(Value::as_u64) == Some(1)).then_some(value))
}

fn read_bounded_rate_limits<R: Read>(mut stdout: R) -> Result<Value, FetchError> {
    let mut chunk = [0_u8; 4096];
    let mut line = Vec::with_capacity(MAX_JSONL_LINE_BYTES.min(4096));
    let mut total_bytes = 0_usize;
    loop {
        let count = stdout.read(&mut chunk).map_err(|_| FetchError::Source)?;
        if count == 0 {
            return response_from_line(&line)?.ok_or(FetchError::InvalidResponse);
        }
        for byte in &chunk[..count] {
            total_bytes = total_bytes
                .checked_add(1)
                .ok_or(FetchError::InvalidResponse)?;
            if total_bytes > MAX_RESPONSE_BYTES {
                return Err(FetchError::InvalidResponse);
            }
            if *byte == b'\n' {
                if let Some(value) = response_from_line(&line)? {
                    return Ok(value);
                }
                line.clear();
            } else {
                if line.len() == MAX_JSONL_LINE_BYTES {
                    return Err(FetchError::InvalidResponse);
                }
                line.push(*byte);
            }
        }
    }
}

fn read_rate_limits() -> Result<Value, FetchError> {
    let stdout = active_child()
        .lock()
        .map_err(|_| FetchError::Source)?
        .as_mut()
        .ok_or(FetchError::Source)?
        .stdout
        .take()
        .ok_or(FetchError::Source)?;
    let (sender, receiver) = mpsc::sync_channel(1);
    thread::spawn(move || {
        let _ = sender.send(read_bounded_rate_limits(stdout));
    });
    receiver
        .recv_timeout(RESPONSE_TIMEOUT)
        .map_err(|_| FetchError::Timeout)?
}

pub fn terminate_active_child() {
    if let Ok(mut active) = active_child().lock() {
        if let Some(child) = active.as_mut() {
            let _ = child.kill();
        }
    }
}

fn finish_active_child() {
    if let Ok(mut active) = active_child().lock() {
        if let Some(mut child) = active.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

pub fn fetch_usage() -> Result<CodexUsage, FetchError> {
    spawn_app_server()?;
    let result = (|| {
        send_request()?;
        let response = read_rate_limits()?;
        if let Some(error) = response.get("error") {
            let message = error.get("message").and_then(Value::as_str).unwrap_or("");
            let lower = message.to_ascii_lowercase();
            if lower.contains("auth") || lower.contains("login") || lower.contains("unauthorized") {
                return Err(FetchError::SignInRequired);
            }
            return Err(FetchError::Source);
        }

        let result = response
            .get("result")
            .cloned()
            .ok_or(FetchError::InvalidResponse)?;
        let parsed: RateLimitsResponse =
            serde_json::from_value(result).map_err(|_| FetchError::InvalidResponse)?;
        let fetched_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        normalize_response(parsed, fetched_at)
    })();

    finish_active_child();
    result
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Cursor, time::Duration};

    use super::{
        executable_from_bin_root, normalize_response, read_bounded_rate_limits,
        terminate_active_child, FetchError, RateLimitsResponse, WindowKind, MAX_JSONL_LINE_BYTES,
        MAX_RESPONSE_BYTES,
    };

    fn response(primary_used: Option<f64>, weekly_used: Option<f64>) -> RateLimitsResponse {
        let primary = primary_used.map(|used_percent| super::SourceWindow {
            used_percent,
            window_duration_mins: Some(300),
            resets_at: Some(1_800_000_000),
        });
        let secondary = weekly_used.map(|used_percent| super::SourceWindow {
            used_percent,
            window_duration_mins: Some(10_080),
            resets_at: Some(1_800_500_000),
        });
        RateLimitsResponse {
            rate_limits: Some(super::RateLimitSnapshot {
                primary,
                secondary,
                credits: Some(super::SourceCredits {
                    has_credits: true,
                    unlimited: false,
                    balance: "420".into(),
                }),
            }),
            rate_limits_by_limit_id: Default::default(),
        }
    }

    #[test]
    fn weekly_is_limiting_at_23_percent_remaining() {
        let usage = normalize_response(response(Some(26.0), Some(77.0)), 123).unwrap();
        assert_eq!(usage.five_hours.unwrap().remaining_percent, 74);
        assert_eq!(usage.weekly.unwrap().remaining_percent, 23);
        assert_eq!(usage.limiting_window.kind, WindowKind::Weekly);
        assert_eq!(usage.limiting_window.remaining_percent, 23);
    }

    #[test]
    fn five_hours_is_limiting_at_9_percent_remaining() {
        let usage = normalize_response(response(Some(91.0), Some(26.0)), 123).unwrap();
        assert_eq!(usage.limiting_window.kind, WindowKind::FiveHours);
        assert_eq!(usage.limiting_window.remaining_percent, 9);
    }

    #[test]
    fn one_missing_window_is_deterministic() {
        let usage = normalize_response(response(None, Some(26.0)), 123).unwrap();
        assert!(usage.five_hours.is_none());
        assert_eq!(usage.limiting_window.kind, WindowKind::Weekly);
        assert_eq!(usage.limiting_window.remaining_percent, 74);
    }

    #[test]
    fn no_supported_windows_is_unavailable() {
        let error = normalize_response(response(None, None), 123).unwrap_err();
        assert!(matches!(error, FetchError::InvalidResponse));
    }

    #[test]
    fn percentages_are_clamped_to_the_valid_range() {
        let low = normalize_response(response(Some(-12.0), None), 123).unwrap();
        let high = normalize_response(response(Some(140.0), None), 123).unwrap();
        assert_eq!(low.limiting_window.remaining_percent, 100);
        assert_eq!(high.limiting_window.remaining_percent, 0);
    }

    #[test]
    fn bounded_reader_accepts_a_normal_response() {
        let payload = b"{\"id\":0,\"result\":{}}\n{\"id\":1,\"result\":{\"rateLimits\":{}}}\n";
        let response = read_bounded_rate_limits(Cursor::new(payload)).unwrap();
        assert_eq!(
            response.get("id").and_then(serde_json::Value::as_u64),
            Some(1)
        );
    }

    #[test]
    fn bounded_reader_rejects_an_oversized_line_without_newline() {
        let payload = vec![b'x'; MAX_JSONL_LINE_BYTES + 1];
        assert!(matches!(
            read_bounded_rate_limits(Cursor::new(payload)),
            Err(FetchError::InvalidResponse)
        ));
    }

    #[test]
    fn bounded_reader_rejects_an_oversized_stream() {
        let payload = vec![b'\n'; MAX_RESPONSE_BYTES + 1];
        assert!(matches!(
            read_bounded_rate_limits(Cursor::new(payload)),
            Err(FetchError::InvalidResponse)
        ));
    }

    #[test]
    fn executable_resolution_uses_a_single_newest_known_install() {
        let root =
            std::env::temp_dir().join(format!("meroa-codex-resolution-{}", std::process::id()));
        let older = root.join("older");
        let newer = root.join("newer");
        fs::create_dir_all(&older).unwrap();
        fs::create_dir_all(&newer).unwrap();
        fs::write(older.join("codex.exe"), b"old").unwrap();
        std::thread::sleep(Duration::from_millis(20));
        fs::write(newer.join("codex.exe"), b"new").unwrap();
        let resolved = executable_from_bin_root(&root).unwrap();
        assert!(resolved.ends_with("newer\\codex.exe") || resolved.ends_with("newer/codex.exe"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn terminating_without_an_active_child_is_safe() {
        terminate_active_child();
    }

    #[test]
    #[ignore = "requires a locally installed and authenticated Codex app-server"]
    fn live_codex_source_exposes_at_least_one_supported_window() {
        let usage = super::fetch_usage().expect("read live Codex usage");
        assert!(usage.five_hours.is_some() || usage.weekly.is_some());
        assert!(usage.limiting_window.remaining_percent <= 100);
    }
}
