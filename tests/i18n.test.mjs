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

test("the hero uses the real limiting value with a neutral remaining label", async () => {
  const app = await readFile(new URL("src/App.tsx", root), "utf8");
  assert.match(app, /const heroLabel = data \? t\("usage\.remaining"\)/);
  assert.match(app, /value=\{data\?\.limitingWindow\.remainingPercent \?\? null\}/);
  assert.doesNotMatch(app, /usage\.limiting/);
});
