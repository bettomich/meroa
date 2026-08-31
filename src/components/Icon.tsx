export type IconName = "refresh" | "more" | "clock" | "calendar" | "cursor" | "reset" | "shield" | "bell" | "settings" | "info" | "power";

interface IconProps {
  name: IconName;
  className?: string;
}

const dotted = {
  fill: "none",
  stroke: "currentColor",
  strokeWidth: 1.9,
  strokeLinecap: "round" as const,
  strokeLinejoin: "round" as const,
  strokeDasharray: "0.2 3.15",
};

const icons: Record<Exclude<IconName, "more">, React.ReactNode> = {
  refresh: <><path {...dotted} d="M20.4 9.4A8.5 8.5 0 1 0 20 16"/><path fill="currentColor" d="m19.7 4.8.9 5.7-5.7-.9 2.1-1.2z"/></>,
  clock: <><circle {...dotted} cx="12" cy="12" r="9"/><circle cx="12" cy="12" r="1.15" fill="currentColor"/><path {...dotted} d="M12 6.8v5.1l3.8 2.2"/></>,
  calendar: <><rect {...dotted} x="3.5" y="5.2" width="17" height="15" rx="2.2"/><path {...dotted} d="M7.6 3.5v4.2m8.8-4.2v4.2M3.8 9.3h16.4"/><g fill="currentColor"><circle cx="8" cy="13" r=".8"/><circle cx="12" cy="13" r=".8"/><circle cx="16" cy="13" r=".8"/><circle cx="8" cy="17" r=".8"/><circle cx="12" cy="17" r=".8"/><circle cx="16" cy="17" r=".8"/></g></>,
  cursor: <><path {...dotted} d="m5 2.8 13.8 8.9-6.1 1.4-2.4 6.1z"/><path {...dotted} d="m13.2 13.1 4.2 5.1"/></>,
  reset: <><circle {...dotted} cx="12" cy="12" r="9"/><path d="M12 6.2v6.2" stroke="currentColor" strokeWidth="1.7" strokeLinecap="round"/><circle cx="12" cy="16.7" r="1" fill="currentColor"/></>,
  shield: <path {...dotted} d="M12 2.8 20 6v5.2c0 5.2-3.2 8.4-8 10-4.8-1.6-8-4.8-8-10V6z"/>,
  bell: <><path {...dotted} d="M5.4 17.2h13.2l-1.5-2.7v-4.2a5.1 5.1 0 0 0-10.2 0v4.2z"/><path {...dotted} d="M9.8 20h4.4"/></>,
  settings: <><circle {...dotted} cx="12" cy="12" r="8.8"/><circle cx="12" cy="12" r="2.8" fill="none" stroke="currentColor" strokeWidth="1.5"/><path {...dotted} d="M12 3v3m0 12v3M3 12h3m12 0h3M5.6 5.6l2.1 2.1m8.6 8.6 2.1 2.1m0-12.8-2.1 2.1m-8.6 8.6-2.1 2.1"/></>,
  info: <><circle {...dotted} cx="12" cy="12" r="9"/><circle cx="12" cy="8" r="1" fill="currentColor"/><path {...dotted} d="M12 11v6"/></>,
  power: <><path {...dotted} d="M7 5.2a8.8 8.8 0 1 0 10 0"/><path d="M12 2.8v9.3" stroke="currentColor" strokeWidth="1.7" strokeLinecap="round"/></>,
};

export function Icon({ name, className = "" }: IconProps) {
  if (name === "more") {
    return <svg className={`icon ${className}`} viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="5" r="1.35"/><circle cx="12" cy="12" r="1.35"/><circle cx="12" cy="19" r="1.35"/></svg>;
  }
  return <svg className={`icon ${className}`} viewBox="0 0 24 24" aria-hidden="true">{icons[name]}</svg>;
}
