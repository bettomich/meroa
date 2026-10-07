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

## Release-signing protection

The designated `release-signing` GitHub Environment is the only location for the two signing secrets once it has been configured. They must not be created as repository secrets. The workflow is split so that the `verify` job runs tests and builds with `contents: read` and no signing material. The signing jobs reference `release-signing` and receive the secrets only in the build step that needs them.

Before adding either secret, configure the environment with these controls:

- required reviewer: the Product Director (`bettomich`);
- deployment branch/tag policy: `main` and the explicitly approved `release/*` tag pattern only;
- do not allow an administrative bypass of protection rules, if the repository settings offer that option.

The workflow is manual only. Signed artifact builds require the `sign_artifacts` input. Publishing requires `publish_release`, a non-empty release tag, Environment approval, and either the `main` branch or a selected `release/*` tag. If a release tag is selected, its name must exactly match `release_tag`. Only the publication job has `contents: write`; it creates a draft release only.

## Private key backup and recovery

Create two encrypted backups of the private key and retain the password separately. Keep one copy in an access-controlled password manager or encrypted vault and a second copy on an encrypted removable drive stored in a separate physical location. Do not put either copy in GitHub, the repository, email, chat, or an unencrypted cloud folder. Record who can access each backup and test the recovery process with a separate non-production key, not by exposing the production key.

If the production key is lost, do not generate a replacement and continue publishing normally: already installed versions trust the embedded public key and will reject artifacts signed by a new key. Recover the original key first. If compromise is suspected, stop releases and plan a signed migration while the original trusted key is still available.

## Tauri signing versus Windows signing

Tauri updater signing verifies that an update was produced by the MEROA release process. Windows Authenticode signing establishes publisher identity for Windows and SmartScreen. They are separate systems. This updater foundation does not add an Authenticode certificate, so SmartScreen warnings may still appear.

## Offline behavior

MEROA continues operating when GitHub is unavailable. Failed automatic checks are silent. A failed manual check shows a compact message in About and can be retried later.

## Safe test strategy

The normal workflow defaults to a non-publishing dry run. It builds signed NSIS artifacts and stores them only as GitHub Actions workflow artifacts. Unit and static tests cover preference defaults, UI states, plugin wiring, and signing configuration.

An end-to-end “update available” test must use a separately approved test build and a test-only updater endpoint serving a correctly signed manifest and installer. It must not overwrite the public `v0.1.0` release or change the production endpoint. Installing that test build over the existing MEROA installation requires Product Director approval immediately before installation.
