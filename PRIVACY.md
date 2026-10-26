# MEROA privacy note

MEROA is local-first. It starts the Codex app-server that is already installed
on the Windows machine, requests the account usage/rate-limit data needed for
the tray view, and keeps that data in memory for the running process.

MEROA does not operate a MEROA cloud endpoint, send telemetry, retrieve
conversations or prompts, read Codex `auth.json` directly, access the
clipboard, or scan arbitrary user files. The only file-system lookup used for
the provider is the trusted local Codex installation location needed to launch
`codex.exe`.

The Codex child process is terminated when MEROA exits. No usage history is
persisted by the V1 application.

MEROA checks the public GitHub Releases endpoint for this repository once every
seven days while the app is running. This sends an ordinary HTTPS request to
GitHub, which receives the IP address and a MEROA version User-Agent; it sends
no Codex usage, conversations, credentials or device identifier. Failed checks
retry no sooner than the following day. Settings can disable automatic checks
or request a manual check. Check timestamps, that preference, and public release
metadata are stored locally in `update-check.v1.json` under the app data folder.

No installer is downloaded or executed by MEROA. After confirmation in Settings,
the official installer download opens in the user's default browser. Downloading
and running the Windows setup remains the user's choice.
