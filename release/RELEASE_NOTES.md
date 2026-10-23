# MEROA 0.2.0

Customizable timer dots, a clearer reset countdown and responsive motion while
preserving MEROA's existing visual identity.

## What's new

- Active dots are black by default. Settings now offers Black, White and Green.
- At 10% remaining or less, active dots turn red regardless of the selected color.
  Consumed dots retain their existing appearance and progress is not reset.
- Selecting Reset hides the dotted ring and caption, showing only a large,
  centered countdown in MEROA's existing typography.
- Screen changes, menu opening/closing, button interactions and timer state
  changes use CSS motion adapted from Material 3 Expressive spring principles.
- A persistent Animations ON/OFF switch is available in Settings and defaults to
  ON. OFF cancels non-essential animation work immediately; the countdown and
  usage refresh continue working.
- The system's reduced-motion preference takes priority over the ON switch and
  is respected when changed while MEROA is running.
- Dot color and animation preferences are stored locally across app restarts.
- Italian and English labels are included for the new settings.

No additional runtime dependencies were introduced.

## Install or update

Download `MEROA_0.2.0_x64-setup.exe` from this release.
For an existing installation, quit MEROA from its menu, run the installer using
the existing installation location, then launch MEROA again.

MEROA does not have an in-app automatic updater.

## Verification

- 20 automated tests pass.
- TypeScript checks and the production frontend build pass.
- Windows x64 release build and NSIS installer generation pass.
- Chromium frontend checks cover colors, the critical threshold, reset display,
  persisted preferences, animations OFF/ON, live reduced motion, keyboard focus
  and refresh behavior with a test-only native-data fixture.

Native WebView runtime behavior and performance on low-powered hardware were
not manually verified. Implementation details are in `docs/MOTION.md`.

## Requirements and notices

- Windows 10/11 x64.
- Codex installed and authenticated.
- The installer is not Authenticode-signed; Windows may show a SmartScreen warning.
- `SHA256SUMS.txt` verifies the installer download. `BUILD_INFO.md` records the
  source commit and build details.
- Third-party notices and font licenses are included with the application.
