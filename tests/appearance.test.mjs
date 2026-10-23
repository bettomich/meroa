import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import test from "node:test";
import { renderToStaticMarkup } from "react-dom/server";
import { createElement } from "react";
import ts from "typescript";

const require = createRequire(import.meta.url);
async function loadModule(path) {
  const source = await readFile(new URL(path, import.meta.url), "utf8");
  const { outputText } = ts.transpileModule(source, {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022, jsx: ts.JsxEmit.ReactJSX },
  });
  const module = { exports: {} };
  new Function("require", "module", "exports", outputText)(require, module, module.exports);
  return module.exports;
}

const preferences = await loadModule("../src/preferences.ts");
const { DottedGauge } = await loadModule("../src/components/DottedGauge.tsx");
const renderGauge = (props) => renderToStaticMarkup(createElement(DottedGauge, { label: "Remaining", ...props }));
const consumed = (html) => html.match(/<circle[^>]*class="gauge__dot"[^>]*>/g) ?? [];
const active = (html) => html.match(/<circle[^>]*class="gauge__dot gauge__dot--active"[^>]*>/g) ?? [];

test("appearance preferences default safely, persist, and tolerate unavailable storage", () => {
  const previousWindow = globalThis.window;
  try {
    const storage = new Map();
    globalThis.window = { localStorage: { getItem: (key) => storage.get(key) ?? null, setItem: (key, value) => storage.set(key, value) } };
    assert.equal(preferences.readDotColor(), "black");
    assert.equal(preferences.readAnimationsEnabled(), true);
    for (const color of ["white", "green", "black"]) {
      preferences.saveDotColor(color);
      assert.equal(preferences.readDotColor(), color);
    }
    for (const enabled of [false, true]) {
      preferences.saveAnimationsEnabled(enabled);
      assert.equal(preferences.readAnimationsEnabled(), enabled);
    }
    storage.set(preferences.DOT_COLOR_STORAGE_KEY, "invalid");
    storage.set(preferences.ANIMATIONS_STORAGE_KEY, "invalid");
    assert.equal(preferences.readDotColor(), "black");
    assert.equal(preferences.readAnimationsEnabled(), true);
    globalThis.window = { get localStorage() { throw new Error("Storage blocked"); } };
    assert.equal(preferences.readDotColor(), "black");
    assert.equal(preferences.readAnimationsEnabled(), true);
    assert.doesNotThrow(() => preferences.saveDotColor("green"));
    assert.doesNotThrow(() => preferences.saveAnimationsEnabled(false));
  } finally {
    if (previousWindow === undefined) delete globalThis.window;
    else globalThis.window = previousWindow;
  }
});

test("color selection and critical warning preserve consumed dots and the original active count", () => {
  for (const value of [100, 50, 10.01, 10, 9, 0, -5, 120, null]) {
    const expectedActive = value === null ? 0 : Math.round(Math.min(100, Math.max(0, value)) / 100 * 42);
    const original = renderGauge({ value });
    for (const dotColor of ["black", "white", "green"]) {
      const html = renderGauge({ value, dotColor });
      assert.equal(active(html).length, expectedActive);
      assert.equal(consumed(html).length, 42 - expectedActive);
      assert.deepEqual(consumed(html), consumed(original));
      const expectedColor = value !== null && value <= 10 ? "critical" : dotColor;
      assert.ok(html.includes(`data-dot-color="${expectedColor}"`));
      assert.ok(!html.includes("gauge__dot--accent"));
    }
  }
});

test("reset hides both the ring and caption, retaining the time and accessible label", () => {
  const html = renderGauge({ value: null, displayValue: "3d 14h", valueKind: "duration", label: "Reset" });
  assert.match(html, /class="gauge__ring-layer" hidden=""/);
  assert.match(html, /class="gauge__label" hidden=""/);
  assert.match(html, /aria-label="3d 14h Reset"/);
  assert.match(html, /gauge__value--duration/);
  assert.match(html, />3d 14h<\/span>/);
  assert.doesNotMatch(renderGauge({ value: 50 }), /hidden=""/);
});

test("the essential countdown still progresses independently of animation preferences", async () => {
  const { selectedHeroMetric, formatReset } = await loadModule("../src/usage.ts");
  const data = { limitingWindow: { resetsAt: 7200 } };
  assert.equal(selectedHeroMetric(data, "reset", 0).value, formatReset(7200, 0));
  assert.equal(selectedHeroMetric(data, "reset", 3600).value, formatReset(7200, 3600));
  assert.notEqual(selectedHeroMetric(data, "reset", 0).value, selectedHeroMetric(data, "reset", 3600).value);
  assert.equal(selectedHeroMetric(data, "reset", 7200).value, "Now");
});
