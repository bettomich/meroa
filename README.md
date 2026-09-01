# MEROA

MEROA is a local, tray-first Windows utility for understanding Codex usage.

The current build includes the approved Milestone 1 shell and the Milestone 2
local data provider. Real 5 Hours, Weekly, reset, and Credits values come from
the installed Codex app-server. The Hero and stable Windows tray icon display
the limiting window. See [the data-source notes](docs/CODEX_USAGE_SOURCE.md).

MEROA does not include Reserve/Pace, alerts, account management, analytics, or
cloud services.

## Development

```powershell
npm install
npm run tauri:dev
```

## Verification

```powershell
npm test
npm run build
npm run tauri:build
```
