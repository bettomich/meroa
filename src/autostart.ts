import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart";

const AUTOSTART_CHOICE_KEY = "meroa.autostart-choice.v1";
const AUTOSTART_DATABASE = "meroa-preferences";
const AUTOSTART_STORE = "preferences";
const AUTOSTART_RECORD = "autostart-choice";

export type AutostartState = "loading" | "enabled" | "disabled" | "error";
type AutostartChoice = "default-applied" | "user-enabled" | "user-disabled";

function localChoice(): AutostartChoice | null {
  try {
    const choice = localStorage.getItem(AUTOSTART_CHOICE_KEY);
    return choice === "default-applied" || choice === "user-enabled" || choice === "user-disabled"
      ? choice
      : null;
  } catch {
    return null;
  }
}

function openPreferenceDatabase(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(AUTOSTART_DATABASE, 1);
    request.onupgradeneeded = () => request.result.createObjectStore(AUTOSTART_STORE);
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}

async function readChoice(): Promise<AutostartChoice | null> {
  try {
    const database = await openPreferenceDatabase();
    const choice = await new Promise<AutostartChoice | null>((resolve, reject) => {
      const request = database.transaction(AUTOSTART_STORE, "readonly")
        .objectStore(AUTOSTART_STORE)
        .get(AUTOSTART_RECORD);
      request.onsuccess = () => resolve(
        request.result === "default-applied" || request.result === "user-enabled" || request.result === "user-disabled"
          ? request.result
          : null,
      );
      request.onerror = () => reject(request.error);
    });
    database.close();
    return choice ?? localChoice();
  } catch {
    return localChoice();
  }
}

async function writeChoice(choice: AutostartChoice): Promise<void> {
  try {
    const database = await openPreferenceDatabase();
    await new Promise<void>((resolve, reject) => {
      const transaction = database.transaction(AUTOSTART_STORE, "readwrite");
      transaction.objectStore(AUTOSTART_STORE).put(choice, AUTOSTART_RECORD);
      transaction.oncomplete = () => resolve();
      transaction.onerror = () => reject(transaction.error);
      transaction.onabort = () => reject(transaction.error);
    });
    database.close();
    try { localStorage.setItem(AUTOSTART_CHOICE_KEY, choice); } catch { /* cache only */ }
  } catch {
    try {
      localStorage.setItem(AUTOSTART_CHOICE_KEY, choice);
    } catch {
      throw new Error("Could not persist the autostart choice");
    }
  }
}

export async function readAutostartState(): Promise<AutostartState> {
  return (await isEnabled()) ? "enabled" : "disabled";
}

export async function loadAutostartState(): Promise<AutostartState> {
  const choice = await readChoice();
  if (choice === null) {
    // Persist before enabling: failure to retain the one-time default must never
    // leave an unrecorded setting that could later be re-applied unexpectedly.
    await writeChoice("default-applied");
    if (!(await isEnabled())) {
      await enable();
    }
  }
  return readAutostartState();
}

export async function setAutostartEnabled(shouldEnable: boolean): Promise<AutostartState> {
  // Persist the explicit intent first. If the system call fails, a later launch
  // reads Windows again rather than automatically reversing the user's choice.
  await writeChoice(shouldEnable ? "user-enabled" : "user-disabled");
  if (shouldEnable) {
    await enable();
  } else {
    await disable();
  }
  const state = await readAutostartState();
  if ((state === "enabled") !== shouldEnable) {
    throw new Error("Autostart state did not change");
  }
  return state;
}
