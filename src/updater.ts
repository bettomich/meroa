import { check, type Update } from "@tauri-apps/plugin-updater";

export const AUTOMATIC_UPDATE_CHECK_KEY = "meroa.automatic-update-check.v1";
export type MeroaUpdate = Update;

export type UpdateState =
  | { status: "idle" }
  | { status: "checking" }
  | { status: "upToDate" }
  | { status: "available"; version: string }
  | { status: "downloading"; version: string; progress: number | null }
  | { status: "installing"; version: string }
  | { status: "error" };

export function readAutomaticUpdateCheck(): boolean {
  try {
    return window.localStorage.getItem(AUTOMATIC_UPDATE_CHECK_KEY) !== "false";
  } catch {
    return true;
  }
}

export function saveAutomaticUpdateCheck(enabled: boolean): void {
  try {
    window.localStorage.setItem(AUTOMATIC_UPDATE_CHECK_KEY, String(enabled));
  } catch {
    // The setting remains active for this session when storage is unavailable.
  }
}

export async function checkForMeroaUpdate(): Promise<Update | null> {
  return check({ timeout: 10_000 });
}

export async function downloadAndInstallMeroaUpdate(
  update: Update,
  onStateChange: (state: UpdateState) => void,
): Promise<void> {
  let downloaded = 0;
  let contentLength: number | undefined;

  await update.downloadAndInstall((event) => {
    if (event.event === "Started") {
      contentLength = event.data.contentLength;
      onStateChange({ status: "downloading", version: update.version, progress: 0 });
      return;
    }

    if (event.event === "Progress") {
      downloaded += event.data.chunkLength;
      const progress = contentLength && contentLength > 0
        ? Math.min(100, Math.round((downloaded / contentLength) * 100))
        : null;
      onStateChange({ status: "downloading", version: update.version, progress });
      return;
    }

    onStateChange({ status: "installing", version: update.version });
  }, {
    timeout: 120_000,
    restartAfterInstall: true,
  });
}
