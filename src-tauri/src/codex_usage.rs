use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    env, fs,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const FIVE_HOURS_MINUTES: u64 = 5 * 60;
const WEEKLY_MINUTES: u64 = 7 * 24 * 60;
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(20);

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
    Unavailable(String),
    Source(String),
}

impl FetchError {
    pub fn message(&self) -> &str {
        match self {
            Self::Unavailable(message) | Self::Source(message) => message,
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
    (100.0 - used_percent.clamp(0.0, 100.0)).round() as u8
}

fn normalize_window(source: &SourceWindow, kind: WindowKind) -> UsageWindow {
    UsageWindow {
        kind,
        used_percent: source.used_percent,
        remaining_percent: remaining_percent(source.used_percent),
        resets_at: source.resets_at,
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
        .ok_or_else(|| {
            FetchError::Unavailable("Codex rate limits are not available for this account.".into())
        })?;

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
        (None, None) => {
            return Err(FetchError::Unavailable(
                "The Codex response did not include a 5-hour or weekly window.".into(),
            ))
        }
    };
    let limiting_window = LimitingWindow {
        kind: limiting.kind,
        remaining_percent: limiting.remaining_percent,
        resets_at: limiting.resets_at,
    };

    let credits = snapshot.credits.map(|credits| Credits {
        has_credits: credits.has_credits,
        unlimited: credits.unlimited,
        balance: credits.balance,
    });

    Ok(CodexUsage {
        five_hours,
        weekly,
        credits,
        limiting_window,
        fetched_at,
    })
}

fn codex_executable() -> Result<PathBuf, FetchError> {
    if let Some(path) = env::var_os("LOCALAPPDATA") {
        let bin_root = PathBuf::from(path).join("OpenAI").join("Codex").join("bin");
        if let Ok(entries) = fs::read_dir(bin_root) {
            let mut candidates: Vec<PathBuf> = entries
                .filter_map(Result::ok)
                .map(|entry| entry.path().join("codex.exe"))
                .filter(|path| path.is_file())
                .collect();
            candidates.sort();
            if let Some(path) = candidates.pop() {
                return Ok(path);
            }
        }
    }

    Ok(PathBuf::from("codex"))
}

fn spawn_app_server() -> Result<std::process::Child, FetchError> {
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

    command.spawn().map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            FetchError::Unavailable("Codex is not installed or is not available on PATH.".into())
        } else {
            FetchError::Source(format!("Could not start Codex app-server: {error}"))
        }
    })
}

fn send_request(child: &mut std::process::Child) -> Result<(), FetchError> {
    let stdin = child
        .stdin
        .as_mut()
        .ok_or_else(|| FetchError::Source("Codex app-server stdin is unavailable.".into()))?;
    for message in [
        r#"{"method":"initialize","id":0,"params":{"clientInfo":{"name":"meroa","title":"MEROA","version":"0.1.0"}}}"#,
        r#"{"method":"initialized","params":{}}"#,
        r#"{"method":"account/rateLimits/read","id":1}"#,
    ] {
        writeln!(stdin, "{message}").map_err(|error| {
            FetchError::Source(format!("Could not query Codex app-server: {error}"))
        })?;
    }
    stdin.flush().map_err(|error| {
        FetchError::Source(format!("Could not flush Codex app-server request: {error}"))
    })
}

fn read_rate_limits(child: &mut std::process::Child) -> Result<Value, FetchError> {
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| FetchError::Source("Codex app-server stdout is unavailable.".into()))?;
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            let Ok(value) = serde_json::from_str::<Value>(&line) else {
                continue;
            };
            if value.get("id").and_then(Value::as_u64) == Some(1) {
                let _ = sender.send(value);
                break;
            }
        }
    });

    receiver
        .recv_timeout(RESPONSE_TIMEOUT)
        .map_err(|_| FetchError::Source("Timed out while reading Codex usage data.".into()))
}

pub fn fetch_usage() -> Result<CodexUsage, FetchError> {
    let mut child = spawn_app_server()?;
    let result = (|| {
        send_request(&mut child)?;
        let response = read_rate_limits(&mut child)?;
        if let Some(error) = response.get("error") {
            let message = error
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("Codex returned an unknown error.");
            let lower = message.to_ascii_lowercase();
            if lower.contains("auth") || lower.contains("login") || lower.contains("unauthorized") {
                return Err(FetchError::Unavailable(
                    "Sign in to Codex with ChatGPT to make usage data available.".into(),
                ));
            }
            return Err(FetchError::Source(format!(
                "Codex app-server error: {message}"
            )));
        }

        let result = response
            .get("result")
            .cloned()
            .ok_or_else(|| FetchError::Source("Codex returned no rate-limit result.".into()))?;
        let parsed: RateLimitsResponse = serde_json::from_value(result).map_err(|error| {
            FetchError::Source(format!("Invalid Codex rate-limit response: {error}"))
        })?;
        let fetched_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        normalize_response(parsed, fetched_at)
    })();

    let _ = child.kill();
    let _ = child.wait();
    result
}

#[cfg(test)]
mod tests {
    use super::{normalize_response, FetchError, RateLimitsResponse, WindowKind};

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
        assert!(matches!(error, FetchError::Unavailable(_)));
    }

    #[test]
    fn percentages_are_clamped_to_the_valid_range() {
        let low = normalize_response(response(Some(-12.0), None), 123).unwrap();
        let high = normalize_response(response(Some(140.0), None), 123).unwrap();
        assert_eq!(low.limiting_window.remaining_percent, 100);
        assert_eq!(high.limiting_window.remaining_percent, 0);
    }

    #[test]
    #[ignore = "requires a locally installed and authenticated Codex app-server"]
    fn live_codex_source_exposes_at_least_one_supported_window() {
        let usage = super::fetch_usage().expect("read live Codex usage");
        assert!(usage.five_hours.is_some() || usage.weekly.is_some());
        assert!(usage.limiting_window.remaining_percent <= 100);
    }
}
