import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export interface AvailableUpdate { version: string; downloadUrl: string; releaseUrl: string }
export interface UpdateState {
  currentVersion: string;
  automaticChecks: boolean;
  checkedAt: number | null;
  lastAttemptAt: number | null;
  nextCheckAt: number;
  available: AvailableUpdate | null;
  checking: boolean;
  error: boolean;
}

export function useUpdates() {
  const [state, setState] = useState<UpdateState | null>(null);
  const [unavailable, setUnavailable] = useState(false);
  useEffect(() => {
    if (!isTauri()) return;
    let active = true;
    let receivedEvent = false;
    let stop: (() => void) | undefined;
    // Subscribe before reading the snapshot so a fast background result is not lost.
    void listen<UpdateState>("update-state", ({ payload }) => {
      receivedEvent = true;
      if (active) setState(payload);
    }).then(async (unlisten) => {
      if (!active) { unlisten(); return; }
      stop = unlisten;
      const initial = await invoke<UpdateState>("get_update_state");
      if (active && !receivedEvent) setState(initial);
    }).catch(() => { if (active) setUnavailable(true); });
    return () => { active = false; stop?.(); };
  }, []);

  return {
    state,
    unavailable: unavailable || !isTauri(),
    checkNow: async () => {
      try { setState(await invoke<UpdateState>("check_for_update")); }
      catch { setUnavailable(true); }
    },
    setAutomaticChecks: async (enabled: boolean) => {
      try { setState(await invoke<UpdateState>("set_automatic_update_checks", { enabled })); }
      catch { setUnavailable(true); }
    },
  };
}

export async function openUpdateDownload(version: string): Promise<void> {
  // The native command reads and validates its own cached release; JS cannot
  // supply an arbitrary URL or invoke an installer.
  await invoke("open_update_download", { version });
}
