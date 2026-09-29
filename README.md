# MEROA

MEROA is a local-first, tray-first Windows utility for understanding Codex
usage. It reads usage windows from the Codex app-server installed on the same
machine and shows 5 Hours, Weekly, Credits, and Reset values.

## V1 usage

- Windows 10/11 with the official Codex app installed and signed in.
- Start MEROA from its executable. It opens quietly in the notification area;
  click the tray icon to open the popover.
- The popover refreshes automatically and the Hero/tray selector follows the
  currently limiting window.
- Windows autostart can be enabled in Settings.
- To exit, open the `···` menu and choose **Esci**, or choose **Quit** from the
  tray context menu.

MEROA does not retrieve conversations or prompts, read `auth.json`, store data
in a MEROA cloud service, or provide Reserve/Pace/Alerts, history, account
management, analytics, or an updater. See [the data-source notes](docs/CODEX_USAGE_SOURCE.md)
and the [privacy note](PRIVACY.md).

Known Windows limitation: the tray icon may be placed under the `^` hidden
icons area. Release binaries are currently unsigned and may show a SmartScreen
warning.

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
