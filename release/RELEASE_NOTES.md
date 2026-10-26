# MEROA 0.3.0

Lightweight weekly update checks with a quiet notification in Settings and a
confirmation before opening the official download.

## What's new

- MEROA checks public GitHub Releases once every seven days while running.
  The first due check starts asynchronously after startup.
- The last check, automatic-check preference and available release are stored
  locally, so restarting MEROA does not trigger repeated requests.
- A small indicator on More options and an update notice in the Settings menu
  show when a newer stable Windows release is available.
- Settings includes the installed version, last successful check, a weekly-check
  ON/OFF switch and a manual Check now button.
- Update opens a confirmation with Open download and Later. Only after consent
  does the official GitHub installer download open in the default browser.
- Automatic checks default ON. OFF suspends scheduled checks; manual checks
  remain available. No frontend polling interval or additional polling thread
  runs between checks.
- Network failures keep any previously found update and retry after 24 hours.
  Requests have a timeout and manual checks have a one-minute cooldown.
- Releases are compared semantically. Drafts, prereleases, older versions,
  missing Windows installers and non-canonical download URLs are rejected.
- Italian and English update messages follow MEROA's existing appearance and
  animation preferences. Timer behavior is unchanged.

The customizable dots, centered Reset display and accessible motion controls
from 0.2.0 are included.

## Install or update

Download `MEROA_0.3.0_x64-setup.exe` from this release.
Quit MEROA from its menu, run the installer using the existing installation
location, then launch MEROA again.

Versions before 0.3.0 need this manual upgrade to gain weekly update notices.
Future notices guide you to the official download; MEROA does not download or
execute an installer automatically. Windows setup remains an explicit user action.

## Verification

- 20 frontend/unit tests and TypeScript production build pass.
- 21 native tests pass; three opt-in tests are skipped by default.
- The opt-in native HTTPS smoke test against real GitHub release metadata passes.
- Windows x64 release compilation and NSIS installer generation pass.
- Chromium frontend checks cover check progress, update availability, persisted
  OFF, cached offers, consent, Later/Escape and focus, errors, animation OFF and
  the existing color/reset controls using test-only native-data fixtures.

No test release is published and no installer is executed during these checks.
Windows upgrade execution and low-powered hardware performance were not manually
verified. See `docs/UPDATE_CHECKS.md` for implementation and privacy details.

## Requirements and notices

- Windows 10/11 x64, with Codex installed and authenticated.
- Weekly checks send public release-metadata requests to GitHub; no Codex usage,
  conversations, credentials or device identifier is sent. See `PRIVACY.md`.
- The installer is not Authenticode-signed; Windows may show a SmartScreen warning.
- `SHA256SUMS.txt` verifies the installer download. `BUILD_INFO.md` records the
  build source commit and validation details.
- Third-party notices and font licenses are included with the application.
