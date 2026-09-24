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
  fill: "none",
  stroke: "currentColor",
  strokeWidth: 1.8,
  strokeLinecap: "round" as const,
  strokeLinejoin: "round" as const,
};

const icons: Record<Exclude<IconName, "more">, React.ReactNode> = {
  refresh: <><path {...dottedStroke} d="M20 9.5a8 8 0 1 0-.4 6.2"/><path fill="currentColor" d="m19.6 5 .9 5.4-5.4-.9 2-1.2z"/></>,
  clock: <><circle {...dottedStroke} cx="12" cy="12" r="8"/><circle cx="12" cy="12" r="1.05" fill="currentColor"/><path {...dottedStroke} d="M12 7.5v4.4l3.25 1.9"/></>,
  calendar: <><rect {...dottedStroke} x="4" y="5.4" width="16" height="14.6" rx="2"/><path {...dottedStroke} d="M8 3.8v3.9m8-3.9v3.9M4.1 9.4h15.8"/><g fill="currentColor"><circle cx="8" cy="13.1" r=".75"/><circle cx="12" cy="13.1" r=".75"/><circle cx="16" cy="13.1" r=".75"/><circle cx="8" cy="16.8" r=".75"/><circle cx="12" cy="16.8" r=".75"/><circle cx="16" cy="16.8" r=".75"/></g></>,
  cursor: <><path {...dottedStroke} d="m5.5 3.5 13 8.4-5.8 1.35-2.25 5.75z"/><path {...dottedStroke} d="m13 13.2 3.9 4.7"/></>,
  reset: <><circle {...dottedStroke} cx="12" cy="12" r="8"/><path d="M12 7v5.5" stroke="currentColor" strokeWidth="1.7" strokeLinecap="round"/><circle cx="12" cy="16.3" r=".95" fill="currentColor"/></>,
  shield: <path {...dottedStroke} d="M12 3.5 19.5 6.4v4.9c0 4.9-3 7.9-7.5 9.2-4.5-1.3-7.5-4.3-7.5-9.2V6.4z"/>,
  bell: <><path {...dottedStroke} d="M5.8 17h12.4l-1.4-2.5v-4.1a4.8 4.8 0 0 0-9.6 0v4.1z"/><path {...dottedStroke} d="M10 19.7h4"/></>,
  settings: <><path {...menuStroke} d="M19.43 12.98c.04-.32.07-.65.07-.98s-.02-.66-.07-.98l2.11-1.65-2-3.46-2.49 1a7.6 7.6 0 0 0-1.69-.98L15 3.35h-4l-.36 2.58c-.6.25-1.16.58-1.69.98l-2.49-1-2 3.46 2.11 1.65c-.04.32-.07.65-.07.98s.02.66.07.98l-2.11 1.65 2 3.46 2.49-1c.52.4 1.08.73 1.69.98L11 20.65h4l.36-2.58a7.6 7.6 0 0 0 1.69-.98l2.49 1 2-3.46-2.11-1.65Z"/><circle {...menuStroke} cx="13" cy="12" r="2.5"/></>,
  info: <><circle {...menuStroke} cx="12" cy="12" r="8.7"/><circle cx="12" cy="7.55" r=".9" fill="currentColor"/><path {...menuStroke} d="M12 11.1v5.35"/></>,
  power: <><path {...menuStroke} d="M10 4.5H5.5v15H10"/><path {...menuStroke} d="M13 12h7.5m-3.5-4 4 4-4 4"/></>,
};

export function Icon({ name, className = "" }: IconProps) {
  if (name === "more") {
    return <svg className={`icon ${className}`} viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><circle cx="12" cy="5" r="1.2"/><circle cx="12" cy="12" r="1.2"/><circle cx="12" cy="19" r="1.2"/></svg>;
  }
  return <svg className={`icon ${className}`} viewBox="0 0 24 24" fill="none" aria-hidden="true">{icons[name]}</svg>;
}
