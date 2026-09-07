export type WindowKind = "fiveHours" | "weekly";

export interface UsageWindow {
  kind: WindowKind;
  usedPercent: number;
  remainingPercent: number;
  resetsAt: number | null;
}

export interface Credits {
  hasCredits: boolean;
  unlimited: boolean;
  balance: string;
}

export interface CodexUsage {
  fiveHours: UsageWindow | null;
  weekly: UsageWindow | null;
  credits: Credits | null;
  limitingWindow: {
    kind: WindowKind;
    remainingPercent: number;
    resetsAt: number | null;
  };
  fetchedAt: number;
}

export type UsageState =
  | { status: "loading" }
  | { status: "available"; data: CodexUsage }
  | { status: "unavailable"; message: string }
  | { status: "error"; message: string }
  | { status: "stale"; data: CodexUsage; message: string };

export function usageData(state: UsageState): CodexUsage | null {
  return state.status === "available" || state.status === "stale" ? state.data : null;
}

export type HeroSelection = WindowKind | "credits" | "reset";
export type TrayMode = "auto" | WindowKind;

export const HERO_SELECTION_STORAGE_KEY = "meroa.hero-selection.v1";
export const TRAY_MODE_STORAGE_KEY = "meroa.tray-mode.v1";
export const STALE_AFTER_SECONDS = 10 * 60;

export function readHeroSelection(): HeroSelection {
  try {
    const value = window.localStorage.getItem(HERO_SELECTION_STORAGE_KEY);
    return value === "fiveHours" || value === "weekly" || value === "credits" || value === "reset"
      ? value
      : "weekly";
  } catch {
    return "weekly";
  }
}

export function saveHeroSelection(selection: HeroSelection): void {
  try {
    window.localStorage.setItem(HERO_SELECTION_STORAGE_KEY, selection);
  } catch {
    // The selector remains usable when storage is unavailable.
  }
}

export interface HeroMetric {
  label: "fiveHours" | "weekly" | "credits" | "reset";
  value: string | null;
  percentage: number | null;
}

export function selectedHeroMetric(
  data: CodexUsage | null,
  selection: HeroSelection,
  nowSeconds = Date.now() / 1000,
): HeroMetric {
  if (selection === "fiveHours") return { label: selection, value: data?.fiveHours ? `${data.fiveHours.remainingPercent}%` : null, percentage: data?.fiveHours?.remainingPercent ?? null };
  if (selection === "weekly") return { label: selection, value: data?.weekly ? `${data.weekly.remainingPercent}%` : null, percentage: data?.weekly?.remainingPercent ?? null };
  if (selection === "credits") return { label: selection, value: formatCredits(data?.credits ?? null), percentage: null };
  return { label: selection, value: formatReset(data?.limitingWindow.resetsAt ?? null, nowSeconds), percentage: null };
}

export function readTrayMode(): TrayMode {
  try {
    const value = window.localStorage.getItem(TRAY_MODE_STORAGE_KEY);
    return value === "fiveHours" || value === "weekly" ? value : "auto";
  } catch {
    return "auto";
  }
}

export function saveTrayMode(mode: TrayMode): void {
  try {
    window.localStorage.setItem(TRAY_MODE_STORAGE_KEY, mode);
  } catch {
    // The tray still uses the selected value for this application session.
  }
}

export function snapshotIsStale(fetchedAt: number, nowSeconds = Date.now() / 1000): boolean {
  return nowSeconds - fetchedAt >= STALE_AFTER_SECONDS;
}

export function relativeUpdatedLabel(
  fetchedAt: number,
  language: "en" | "it",
  nowSeconds = Date.now() / 1000,
): string {
  const minutes = Math.max(0, Math.floor((nowSeconds - fetchedAt) / 60));
  if (minutes === 0) return language === "it" ? "Aggiornato ora" : "Updated now";
  if (language === "it") return `Aggiornato ${minutes} min fa`;
  return `Updated ${minutes} min ago`;
}

export function formatReset(resetsAt: number | null, nowSeconds = Date.now() / 1000): string {
  if (resetsAt === null) return "—";
  const remainingSeconds = Math.max(0, Math.ceil(resetsAt - nowSeconds));
  if (remainingSeconds === 0) return "Now";
  const days = Math.floor(remainingSeconds / 86_400);
  const hours = Math.floor((remainingSeconds % 86_400) / 3_600);
  const minutes = Math.max(1, Math.ceil((remainingSeconds % 3_600) / 60));
  if (days > 0) return `${days}d ${hours}h`;
  if (hours > 0) return `${hours}h ${minutes}m`;
  return `${minutes}m`;
}

export function formatCredits(credits: Credits | null): string {
  if (!credits) return "—";
  if (credits.unlimited) return "∞";
  return credits.balance;
}
