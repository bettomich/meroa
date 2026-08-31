# MEROA

MEROA is a local, tray-first Windows utility for understanding and planning Codex usage.

This repository currently contains **Milestone 1** only: the Tauri 2 shell, dynamic tray indicator, compact React popover, mock usage data, and the Italian/English localization structure. It intentionally does not include the real Codex provider, Reserve/Pace, alerts, or cloud services.

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
