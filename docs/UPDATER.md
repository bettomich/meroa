# MEROA updater architecture

MEROA uses the official Tauri 2 updater on Windows x64. The updater is independent from the Codex usage provider:

```text
About / update controls
        ↓
src/updater.ts
        ↓
Tauri updater plugin
        ↓
GitHub Releases (latest.json + signed NSIS installer)
```

## Runtime behavior

- The endpoint is `https://github.com/bettomich/meroa/releases/latest/download/latest.json`.
- Automatic checks are enabled by default and start asynchronously after the application has opened.
- The preference is stored locally under `meroa.automatic-update-check.v1`.
- The existing application identifier remains `com.meroa.desktop`, so WebView data is retained when NSIS installs a newer version over the current installation. Language, hero metric, tray mode, refresh behavior, and autostart records keep their existing storage locations; language selection is now explicitly persisted as `meroa.language.v1`.
- Manual checks and installation are available in About.
- Network, metadata, download, signature, and installer failures are contained within the updater UI. They do not stop usage refresh, the tray, or the Codex provider.
- On Windows, the official plugin launches the NSIS updater, closes MEROA, and requests an application restart after installation.

## Signing and release artifacts

`bundle.createUpdaterArtifacts` is enabled. A Windows NSIS release build produces the installer and its Tauri updater signature. `tauri-apps/tauri-action` produces `latest.json` for the GitHub Release and is configured to prefer NSIS metadata.

The public updater key is embedded in `tauri.conf.json`. The private key and its password must only be supplied to release builds through these GitHub Actions Secrets:

- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

Losing the private updater key prevents publishing updates that existing updater-enabled installations can trust. The private key must never be committed, attached to a release, or printed in logs.

## Tauri signing versus Windows signing

Tauri updater signing verifies that an update was produced by the MEROA release process. Windows Authenticode signing establishes publisher identity for Windows and SmartScreen. They are separate systems. This updater foundation does not add an Authenticode certificate, so SmartScreen warnings may still appear.

## Offline behavior

MEROA continues operating when GitHub is unavailable. Failed automatic checks are silent. A failed manual check shows a compact message in About and can be retried later.

## Safe test strategy

The normal workflow defaults to a non-publishing dry run. It builds signed NSIS artifacts and stores them only as GitHub Actions workflow artifacts. Unit and static tests cover preference defaults, UI states, plugin wiring, and signing configuration.

An end-to-end “update available” test must use a separately approved test build and a test-only updater endpoint serving a correctly signed manifest and installer. It must not overwrite the public `v0.1.0` release or change the production endpoint. Installing that test build over the existing MEROA installation requires Product Director approval immediately before installation.
