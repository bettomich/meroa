import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

const root = new URL("../", import.meta.url);

async function readJson(path) {
  return JSON.parse(await readFile(new URL(path, root), "utf8"));
}

function keys(value, prefix = "") {
  return Object.entries(value).flatMap(([key, child]) => {
    const path = prefix ? `${prefix}.${key}` : key;
    return child && typeof child === "object" ? keys(child, path) : [path];
  });
}

test("English and Italian expose identical translation keys", async () => {
  const [en, it] = await Promise.all([
    readJson("src/locales/en.json"),
    readJson("src/locales/it.json"),
  ]);
  assert.deepEqual(keys(it).sort(), keys(en).sort());
});

test("the production UI is wired to the native Codex usage provider", async () => {
  const [app, provider] = await Promise.all([
    readFile(new URL("src/App.tsx", root), "utf8"),
    readFile(new URL("src-tauri/src/codex_usage.rs", root), "utf8"),
  ]);
  assert.doesNotMatch(app, /mockUsage|mocks\/usage/);
  assert.match(app, /invoke<UsageState>\("refresh_usage"\)/);
  assert.match(provider, /account\/rateLimits\/read/);
});

test("metric cards select the persisted hero metric without a selector bar", async () => {
  const [app, usage] = await Promise.all([
    readFile(new URL("src/App.tsx", root), "utf8"),
    readFile(new URL("src/usage.ts", root), "utf8"),
  ]);
  assert.match(app, /selectedHeroMetric\(data, heroSelection, nowSeconds\)/);
  assert.match(app, /onSelect=\{\(\) => selectHero\("fiveHours"\)\}/);
  assert.match(app, /onSelect=\{\(\) => selectHero\("weekly"\)\}/);
  assert.match(app, /onSelect=\{\(\) => selectHero\("credits"\)\}/);
  assert.match(app, /onSelect=\{\(\) => selectHero\("reset"\)\}/);
  assert.doesNotMatch(app, /hero-selector/);
  assert.match(usage, /meroa\.hero-selection\.v1/);
  assert.match(usage, /: "weekly"/);
});

test("refresh and freshness use the last valid snapshot without mock data", async () => {
  const [app, usage, backend] = await Promise.all([
    readFile(new URL("src/App.tsx", root), "utf8"),
    readFile(new URL("src/usage.ts", root), "utf8"),
    readFile(new URL("src-tauri/src/lib.rs", root), "utf8"),
  ]);
  assert.match(app, /relativeUpdatedLabel\(data\.fetchedAt, language, nowSeconds\)/);
  assert.match(usage, /STALE_AFTER_SECONDS = 10 \* 60/);
  assert.match(backend, /AUTO_REFRESH_INTERVAL: Duration = Duration::from_secs\(5 \* 60\)/);
  assert.match(backend, /refresh_in_progress/);
  assert.match(backend, /reset_auto_refresh\(&store\)/);
});

test("the tray renders one selected percentage and retains both quotas in its localized tooltip", async () => {
  const backend = await readFile(new URL("src-tauri/src/lib.rs", root), "utf8");
  assert.match(backend, /render_tray_icon\(value, size\)/);
  assert.match(backend, /MEROA\\n5 ore: \{five_hours\}%\\nSettimanale: \{weekly\}%/);
  assert.match(backend, /MEROA\\n5 hours: \{five_hours\}%\\nWeekly: \{weekly\}%/);
  assert.match(backend, /TrayMode::Auto/);
  assert.match(backend, /TrayMode::FiveHours/);
  assert.match(backend, /TrayMode::Weekly/);
  assert.doesNotMatch(backend, /weekly_width|separator_width/);
});

test("visual controls share the local icon system and secondary icon token", async () => {
  const [icon, metricIcon, styles] = await Promise.all([
    readFile(new URL("src/components/Icon.tsx", root), "utf8"),
    readFile(new URL("src/components/MetricIcon.tsx", root), "utf8"),
    readFile(new URL("src/styles/global.css", root), "utf8"),
  ]);
  assert.match(icon, /fill="currentColor"/);
  assert.doesNotMatch(icon, /#000|black/);
  assert.match(metricIcon, /className="metric-icon"/);
  assert.match(styles, /--icon-secondary: var\(--text-secondary\)/);
  assert.match(styles, /\.icon-button \{[\s\S]*?color: var\(--icon-secondary\)/);
});

test("every metric value uses the shared Space Grotesk typography with a readable percent unit", async () => {
  const [metricRow, styles, gauge] = await Promise.all([
    readFile(new URL("src/components/MetricRow.tsx", root), "utf8"),
    readFile(new URL("src/styles/global.css", root), "utf8"),
    readFile(new URL("src/components/DottedGauge.tsx", root), "utf8"),
  ]);
  assert.match(metricRow, /function MetricValue/);
  assert.match(metricRow, /metric-row__value-unit/);
  assert.match(styles, /\.metric-row__value \{[\s\S]*?font-family: "Space Grotesk"/);
  assert.match(styles, /font-variant-numeric: tabular-nums/);
  assert.match(styles, /\.metric-row__value-unit \{[\s\S]*?font-size: \.85em/);
  assert.match(gauge, /"gauge__value"/);
});
