export interface MockUsageSnapshot {
  readonly remaining: number;
  readonly fiveHours: number;
  readonly weekly: number;
  readonly credits: number;
  readonly status: "safe";
}

export const mockUsage: MockUsageSnapshot = Object.freeze({
  remaining: 74,
  fiveHours: 61,
  weekly: 74,
  credits: 420,
  status: "safe",
});
