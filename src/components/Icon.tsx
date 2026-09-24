export type IconName = "refresh" | "more" | "clock" | "calendar" | "cursor" | "reset" | "shield" | "bell" | "settings" | "info" | "power";

interface IconProps {
  name: IconName;
  className?: string;
}

const dottedStroke = {
  fill: "none",
  stroke: "currentColor",
  strokeWidth: 1.8,
  strokeLinecap: "round" as const,
  strokeLinejoin: "round" as const,
  strokeDasharray: "0.2 3",
};

const menuStroke = {
  ...dottedStroke,
  strokeWidth: 1.7,
  strokeDasharray: "0.18 2.65",
};

const icons: Record<Exclude<IconName, "more">, React.ReactNode> = {
  refresh: <><path {...dottedStroke} d="M20 9.5a8 8 0 1 0-.4 6.2"/><path fill="currentColor" d="m19.6 5 .9 5.4-5.4-.9 2-1.2z"/></>,
  clock: <><circle {...dottedStroke} cx="12" cy="12" r="8"/><circle cx="12" cy="12" r="1.05" fill="currentColor"/><path {...dottedStroke} d="M12 7.5v4.4l3.25 1.9"/></>,
  calendar: <><rect {...dottedStroke} x="4" y="5.4" width="16" height="14.6" rx="2"/><path {...dottedStroke} d="M8 3.8v3.9m8-3.9v3.9M4.1 9.4h15.8"/><g fill="currentColor"><circle cx="8" cy="13.1" r=".75"/><circle cx="12" cy="13.1" r=".75"/><circle cx="16" cy="13.1" r=".75"/><circle cx="8" cy="16.8" r=".75"/><circle cx="12" cy="16.8" r=".75"/><circle cx="16" cy="16.8" r=".75"/></g></>,
  cursor: <><path {...dottedStroke} d="m5.5 3.5 13 8.4-5.8 1.35-2.25 5.75z"/><path {...dottedStroke} d="m13 13.2 3.9 4.7"/></>,
  reset: <><circle {...dottedStroke} cx="12" cy="12" r="8"/><path d="M12 7v5.5" stroke="currentColor" strokeWidth="1.7" strokeLinecap="round"/><circle cx="12" cy="16.3" r=".95" fill="currentColor"/></>,
  shield: <path {...dottedStroke} d="M12 3.5 19.5 6.4v4.9c0 4.9-3 7.9-7.5 9.2-4.5-1.3-7.5-4.3-7.5-9.2V6.4z"/>,
  bell: <><path {...dottedStroke} d="M5.8 17h12.4l-1.4-2.5v-4.1a4.8 4.8 0 0 0-9.6 0v4.1z"/><path {...dottedStroke} d="M10 19.7h4"/></>,
  settings: <><circle {...menuStroke} cx="12" cy="12" r="7.35"/><circle {...menuStroke} cx="12" cy="12" r="2.7"/><path {...menuStroke} d="M12 3.6v2.1m0 12.6v2.1M3.6 12h2.1m12.6 0h2.1M6.05 6.05l1.55 1.55m8.8 8.8 1.55 1.55m0-11.9-1.55 1.55m-8.8 8.8-1.55 1.55"/></>,
  info: <><circle {...menuStroke} cx="12" cy="12" r="7.5"/><circle cx="12" cy="8.2" r=".9" fill="currentColor"/><path {...menuStroke} d="M12 11.15v5.15"/></>,
  power: <><path {...menuStroke} d="M5.5 8.7V4.5h7v4.2m0 6.6v4.2h-7V15.3"/><path {...menuStroke} d="M10.4 12h8.1m-3-3 3 3-3 3"/></>,
};

export function Icon({ name, className = "" }: IconProps) {
  if (name === "more") {
    return <svg className={`icon ${className}`} viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><circle cx="12" cy="5" r="1.2"/><circle cx="12" cy="12" r="1.2"/><circle cx="12" cy="19" r="1.2"/></svg>;
  }
  return <svg className={`icon ${className}`} viewBox="0 0 24 24" fill="none" aria-hidden="true">{icons[name]}</svg>;
}
