import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

const root = new URL("../", import.meta.url);
const read = (path) => readFile(new URL(path, root), "utf8");

test("updater checks are opt-out, asynchronous, and isolated from usage refresh", async () => {
  const [app, updater] = await Promise.all([read("src/App.tsx"), read("src/updater.ts")]);

  assert.match(updater, /meroa\.automatic-update-check\.v1/);
  assert.match(updater, /getItem\(AUTOMATIC_UPDATE_CHECK_KEY\) !== "false"/);
  assert.match(updater, /check\(\{ timeout: 10_000 \}\)/);
  assert.match(app, /setTimeout\(\(\) => void runUpdateCheck\(true\), 3_000\)/);
  assert.match(app, /silent \? \{ status: "idle" \} : \{ status: "error" \}/);
  assert.doesNotMatch(updater, /refresh_usage|UsageState|codex/i);
});

test("Windows installation reports progress and requests restart", async () => {
  const updater = await read("src/updater.ts");

  assert.match(updater, /downloadAndInstall/);
  assert.match(updater, /event\.event === "Started"/);
  assert.match(updater, /event\.event === "Progress"/);
  assert.match(updater, /restartAfterInstall: true/);
  assert.match(updater, /timeout: 120_000/);
});

test("Tauri updater uses signed NSIS artifacts from GitHub Releases", async () => {
  const [configurationText, capabilityText, cargo] = await Promise.all([
    read("src-tauri/tauri.conf.json"),
    read("src-tauri/capabilities/default.json"),
    read("src-tauri/Cargo.toml"),
  ]);
  const configuration = JSON.parse(configurationText);
  const capability = JSON.parse(capabilityText);

  assert.equal(configuration.bundle.createUpdaterArtifacts, true);
  assert.deepEqual(configuration.bundle.targets, ["nsis"]);
  assert.equal(configuration.plugins.updater.windows.installMode, "passive");
  assert.deepEqual(configuration.plugins.updater.endpoints, [
    "https://github.com/bettomich/meroa/releases/latest/download/latest.json",
  ]);
  assert.ok(configuration.plugins.updater.pubkey.length > 100);
  assert.doesNotMatch(configuration.plugins.updater.pubkey, /placeholder|example|todo/i);
  assert.ok(capability.permissions.includes("updater:default"));
  assert.match(cargo, /tauri-plugin-updater = "2"/);
});

test("release workflow isolates signing and publication from verification", async () => {
  const workflow = await read(".github/workflows/windows-release.yml");

  assert.match(workflow, /workflow_dispatch:/);
  assert.match(workflow, /sign_artifacts:[\s\S]*?default: false/);
  assert.match(workflow, /publish_release:[\s\S]*?default: false/);
  assert.match(workflow, /Verify source without signing material/);
  assert.match(workflow, /name: release-signing/);
  assert.match(workflow, /sign-windows-x64:[\s\S]*?contents: read/);
  assert.match(workflow, /publish-release:[\s\S]*?contents: write/);
  assert.match(workflow, /Publication is allowed only from main or from a release\/\* tag/);
  assert.match(workflow, /persist-credentials: false/);
  assert.match(workflow, /TAURI_SIGNING_PRIVATE_KEY/);
  assert.match(workflow, /TAURI_SIGNING_PRIVATE_KEY_PASSWORD/);
  assert.match(workflow, /releaseDraft: true/);
  assert.match(workflow, /updaterJsonPreferNsis: true/);
  assert.match(workflow, /uploadUpdaterJson: true/);
  assert.doesNotMatch(workflow, /push:\s*\n/);
});

test("privacy documentation discloses GitHub update traffic without changing local-first claims", async () => {
  const privacy = await read("PRIVACY.md");

  assert.match(privacy, /GitHub Releases/);
  assert.match(privacy, /do not include MEROA telemetry/);
  assert.match(privacy, /not[\s\S]*a MEROA account, analytics, storage, or application backend/);
});

test("the updater-compatible settings preserve the selected language and existing preference keys", async () => {
  const [i18n, usage, autostart, updater] = await Promise.all([
    read("src/i18n/I18nProvider.tsx"),
    read("src/usage.ts"),
    read("src/autostart.ts"),
    read("src/updater.ts"),
  ]);

  assert.match(i18n, /meroa\.language\.v1/);
  assert.match(i18n, /localStorage\.setItem\(LANGUAGE_STORAGE_KEY, nextLanguage\)/);
  assert.match(usage, /meroa\.hero-selection\.v1/);
  assert.match(usage, /meroa\.tray-mode\.v1/);
  assert.match(autostart, /meroa\.autostart-choice\.v1/);
  assert.match(updater, /meroa\.automatic-update-check\.v1/);
});
