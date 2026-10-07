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

Updater-enabled versions contact GitHub Releases for `bettomich/meroa` when
automatic update checks are enabled or when the user starts a manual check.
MEROA reads update metadata and downloads the signed installer only after the
user chooses to update. These requests do not include MEROA telemetry, prompts,
conversations, source code, documents, or a MEROA user identifier.

GitHub is the distribution host for update metadata and installers. It is not
a MEROA account, analytics, storage, or application backend.
