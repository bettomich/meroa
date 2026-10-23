export type DotColor = "black" | "white" | "green";

export const DOT_COLOR_STORAGE_KEY = "meroa.dot-color.v1";
export const ANIMATIONS_STORAGE_KEY = "meroa.animations.v1";

function readPreference(key: string): string | null {
  try {
    return window.localStorage.getItem(key);
  } catch {
    return null;
  }
}

function savePreference(key: string, value: string): void {
  try {
    window.localStorage.setItem(key, value);
  } catch {
    // Preferences still take effect for the current session if storage is unavailable.
  }
}

export function readDotColor(): DotColor {
  const value = readPreference(DOT_COLOR_STORAGE_KEY);
  return value === "white" || value === "green" ? value : "black";
}

export function saveDotColor(color: DotColor): void {
  savePreference(DOT_COLOR_STORAGE_KEY, color);
}

export function readAnimationsEnabled(): boolean {
  return readPreference(ANIMATIONS_STORAGE_KEY) !== "false";
}

export function saveAnimationsEnabled(enabled: boolean): void {
  savePreference(ANIMATIONS_STORAGE_KEY, String(enabled));
}
