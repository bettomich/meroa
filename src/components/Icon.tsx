export type IconName = "refresh" | "more" | "clock" | "calendar" | "cursor" | "reset" | "shield" | "bell" | "settings" | "info" | "power";

interface IconProps {
  name: IconName;
  className?: string;
}

const paths: Record<Exclude<IconName, "more">, React.ReactNode> = {
  refresh: <><path d="M20 11a8 8 0 1 0 2 5"/><path d="M20 5v6h-6"/></>,
  clock: <><circle cx="12" cy="12" r="8.5"/><path d="M12 7.5v5l3.5 2"/></>,
  calendar: <><rect x="4" y="5.5" width="16" height="14" rx="2"/><path d="M8 3.5v4M16 3.5v4M4 9.5h16M8 13h.01M12 13h.01M16 13h.01M8 16.5h.01M12 16.5h.01M16 16.5h.01"/></>,
  cursor: <><path d="m6 3 12 8-5.3 1.5L10 18Z"/><path d="m13 13 4 5"/></>,
  reset: <><circle cx="12" cy="12" r="8.5"/><path d="M12 7.5v5M12 16.5h.01"/></>,
  shield: <path d="M12 3 20 6v5c0 5-3.2 8.3-8 10-4.8-1.7-8-5-8-10V6Z"/>,
  bell: <><path d="M6 17h12l-1.5-2.5V10a4.5 4.5 0 0 0-9 0v4.5Z"/><path d="M10 20h4"/></>,
  settings: <><circle cx="12" cy="12" r="3"/><path d="M19 13.5v-3l-2-.6-.7-1.6 1-1.9-2.1-2.1-1.9 1-1.6-.7-.6-2h-3l-.6 2-1.6.7-1.9-1L2 6.4l1 1.9-.7 1.6-2 .6v3l2 .6.7 1.6-1 1.9L4.1 20l1.9-1 1.6.7.6 2h3l.6-2 1.6-.7 1.9 1 2.1-2.1-1-1.9.7-1.6Z"/></>,
  info: <><circle cx="12" cy="12" r="9"/><path d="M12 10.5V17M12 7h.01"/></>,
  power: <><path d="M12 3v9"/><path d="M7 5.8a8 8 0 1 0 10 0"/></>,
};

export function Icon({ name, className = "" }: IconProps) {
  if (name === "more") {
    return <svg className={`icon ${className}`} viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="5" r="1.4"/><circle cx="12" cy="12" r="1.4"/><circle cx="12" cy="19" r="1.4"/></svg>;
  }
  return (
    <svg className={`icon ${className}`} viewBox="0 0 24 24" aria-hidden="true" fill="none" stroke="currentColor" strokeWidth="1.55" strokeLinecap="round" strokeLinejoin="round">
      {paths[name]}
    </svg>
  );
}
