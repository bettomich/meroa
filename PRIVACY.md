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
