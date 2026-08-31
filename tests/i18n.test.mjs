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

test("the approved mock values remain explicit development data", async () => {
  const source = await readFile(new URL("src/mocks/usage.ts", root), "utf8");
  assert.match(source, /remaining:\s*74/);
  assert.match(source, /fiveHours:\s*61/);
  assert.match(source, /weekly:\s*74/);
  assert.match(source, /credits:\s*420/);
});
