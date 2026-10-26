# Lightweight weekly update checks

This implementation checks public GitHub Release metadata. It does not merge or
enable the separate `feature/updater-foundation` signed-updater pipeline.

## User flow

- After startup, the first due check starts asynchronously after 20 seconds.
- A successful check schedules the next check seven days later; timestamps and
  the available release are cached across app restarts.
- When an update exists, a quiet indicator appears on More options and an
  `Update available` note appears under Settings in the menu.
- Settings shows installed/latest versions, the last successful check, an
  automatic-check toggle and a `Check now` button.
- `Update…` opens an inline confirmation. Only `Open download` invokes the native
  command to open the verified GitHub installer URL in the default browser.
  `Later` or Escape cancels without downloads or effects on the timer.
- The user runs the downloaded NSIS installer. There is no unattended install,
  installer execution or automatic application restart.

## Scheduling and performance

`src-tauri/src/updates.rs` owns a single async task on Tauri's existing Tokio
runtime. It awaits either its next deadline or an explicit settings change;
there is no frontend interval and no additional polling thread. A sleeping task
does no CPU work. The HTTP client exists only during a due check and is dropped
afterward. GitHub receives one bounded metadata request, not an installer download.

Automatic checks default ON. OFF suspends the task until an explicit change;
manual checks remain available. Requests time out after 12 seconds. Failed checks
retain previous availability and retry after 24 hours; manual requests have a
60-second cooldown and a shared in-flight guard prevents duplicates. Startup
never blocks usage loading. When the app is closed nothing runs; an overdue
check happens at the next launch.

Network access uses reqwest with Windows native TLS and system proxy support.
Tokio, reqwest and semver were already in the resolved Rust graph; they are now
direct dependencies for HTTP, scheduling and correct version comparisons. TLS
support is enabled for the runtime request. There are no new JavaScript packages.

## Validation and boundaries

Only published stable releases newer than the running package are offered.
Their exact `MEROA_<semver>_x64-setup.exe` asset must be uploaded and match the
canonical HTTPS download URL for `bettomich/meroa`. Redirects during the API
request are disabled; metadata is capped at 256 KiB. Download URLs are rebuilt
in Rust from a strict stable semver, never accepted from JavaScript. Cached data
is bounded, schema checked, and normalized to drop already-installed or tampered
updates. Clock changes cannot postpone checks indefinitely.

The existing signed updater branch remains separate. A future signed in-app
installer can reuse this notification flow after its release-signing pipeline
is operational. This implementation works with ordinary existing NSIS releases.

References:

- [GitHub latest release API](https://docs.github.com/en/rest/releases/releases#get-the-latest-release)
- [Tokio sleep](https://docs.rs/tokio/1.53.1/tokio/time/fn.sleep.html)
- [reqwest 0.13.4](https://docs.rs/reqwest/0.13.4/reqwest/)

## Verification

- `npm test`: 20 passing frontend/unit checks, including locale parity and the
  existing timer/motion behavior. TypeScript and the production Vite build pass.
- The Windows release build and NSIS installer generation pass for 0.3.0.
- `cargo test --release --lib`: 21 passing native tests, with three explicit
  opt-in tests skipped by default. New tests cover weekly cadence, restart cache,
  cooldown/concurrency, OFF/manual operation, semantic versions, unsafe URLs,
  missing installers, clock adjustments and offline backoff/retained availability.
- The opt-in `live_github_metadata_uses_native_tls_and_matches_the_official_installer`
  test passes against real GitHub over native HTTPS; it downloads metadata only.
- Chromium frontend QA with a test-only IPC fixture covers check progress,
  available update, menu indicators, confirmation, Later/Escape and focus,
  explicit download consent, errors, persisted OFF, animation OFF, existing
  color/reset controls and native popover size limits. No console warnings/errors.

Browser artifacts use a hypothetical 0.4.0 release fixture. No fake release is
published and no installer is downloaded or executed during tests. Windows
installer execution/upgrade and measurements on low-powered hardware are not
part of these checks.
