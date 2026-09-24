import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";

const AUTOSTART_CHOICE_KEY = "meroa.autostart-choice.v1";

export type AutostartState = "loading" | "enabled" | "disabled" | "error";

export async function loadAutostartState(): Promise<AutostartState> {
  let enabled = await isEnabled();
  if (localStorage.getItem(AUTOSTART_CHOICE_KEY) === null) {
    if (!enabled) {
      await enable();
      enabled = await isEnabled();
      if (!enabled) throw new Error("Autostart was not enabled");
    }
    localStorage.setItem(AUTOSTART_CHOICE_KEY, "default-applied");
  }
  return enabled ? "enabled" : "disabled";
}

export async function setAutostartEnabled(shouldEnable: boolean): Promise<AutostartState> {
  if (shouldEnable) await enable();
  else await disable();
  const enabled = await isEnabled();
  if (enabled !== shouldEnable) throw new Error("Autostart state did not change");
  localStorage.setItem(AUTOSTART_CHOICE_KEY, "user-choice");
  return enabled ? "enabled" : "disabled";
}
