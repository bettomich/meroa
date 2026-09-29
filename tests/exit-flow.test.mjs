import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import ts from "typescript";

const root = new URL("../", import.meta.url);

async function loadTypeScriptModule(path) {
  const source = await readFile(new URL(path, root), "utf8");
  const { outputText } = ts.transpileModule(source, {
    compilerOptions: { module: ts.ModuleKind.ESNext, target: ts.ScriptTarget.ES2022 },
  });
  return import(`data:text/javascript;base64,${Buffer.from(outputText).toString("base64")}`);
}

test("internal Exit waits for the native invoke before closing the menu", async () => {
  const { handleQuickMenuAction } = await loadTypeScriptModule("src/menuActions.ts");
  const sequence = [];
  let releaseQuit;
  const quitGate = new Promise((resolve) => { releaseQuit = resolve; });

  const action = handleQuickMenuAction("quit", "quit", {
    onNavigate: () => sequence.push("navigate"),
    onQuit: async () => {
      sequence.push("invoke-started");
      await quitGate;
      sequence.push("invoke-finished");
    },
    onClose: () => sequence.push("menu-closed"),
  });

  await new Promise((resolve) => setImmediate(resolve));
  assert.deepEqual(sequence, ["invoke-started"]);
  releaseQuit();
  await action;
  assert.deepEqual(sequence, ["invoke-started", "invoke-finished", "menu-closed"]);
});

test("native quit invoke uses the registered command and propagates failures", async () => {
  const { requestNativeQuit } = await loadTypeScriptModule("src/quit.ts");
  const commands = [];
  await requestNativeQuit(async (command) => { commands.push(command); });
  assert.deepEqual(commands, ["quit_application_command"]);
  await assert.rejects(
    requestNativeQuit(async () => { throw new Error("native exit unavailable"); }),
    /native exit unavailable/,
  );
});
