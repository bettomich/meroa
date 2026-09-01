# Codex usage data source

MEROA reads real ChatGPT-managed Codex limits through the locally installed
`codex app-server`. It uses the documented JSON-RPC method
`account/rateLimits/read`; it does not call an inferred web endpoint and does
not read Codex authentication files.

Official protocol documentation:
<https://developers.openai.com/codex/app-server>

## Mapping

- A window with `windowDurationMins = 300` is displayed as **5 Hours**.
- A window with `windowDurationMins = 10080` is displayed as **Weekly**.
- `remainingPercent` is `100 - usedPercent`, clamped to `0...100` and rounded
  to the nearest integer for the compact UI and tray raster.
- The limiting window is the available window with the lowest remaining
  percentage. A tie deterministically selects 5 Hours.
- `credits` is displayed as its own balance and never participates in limiting
  window selection.
- `rateLimitResetCredits` is a separate reset entitlement and is not presented
  as the Credits balance.

## Runtime and security

MEROA starts the official local Codex app-server with fixed arguments and uses
its managed ChatGPT authentication. No API key, cookie, access token, or session
token is accepted by the UI, logged, persisted, or committed by MEROA.

The first read happens at startup. MEROA then refreshes every five minutes and
on explicit refresh. If a refresh fails after a successful read, the last value
is marked stale rather than current. Without a valid prior value, the UI and
tray show unavailable or error state instead of simulated data.

## Availability constraints

The integration requires a locally installed Codex CLI version that supports
`app-server` and `account/rateLimits/read`, plus an account authentication mode
supported by Codex services. The app-server CLI is currently documented as an
experimental integration surface, so protocol compatibility is validated by
tests and failures remain non-fatal.
