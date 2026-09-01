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
