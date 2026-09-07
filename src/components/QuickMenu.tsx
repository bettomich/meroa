import { useEffect, useRef } from "react";
import { Icon, type IconName } from "./Icon";
import { useI18n } from "../i18n/I18nProvider";
import type { TrayMode } from "../usage";

interface QuickMenuProps {
  onClose: () => void;
  onRefresh: () => void;
  trayMode: TrayMode;
  onTrayModeChange: (mode: TrayMode) => void;
}

type MenuItem = { key: string; icon: IconName; action?: "refresh"; separator?: boolean };

const items: MenuItem[] = [
  { key: "protectUsage", icon: "shield" },
  { key: "alerts", icon: "bell" },
  { key: "refresh", icon: "refresh", action: "refresh" },
  { key: "settings", icon: "settings", separator: true },
  { key: "about", icon: "info" },
  { key: "quit", icon: "power", separator: true },
];

export function QuickMenu({ onClose, onRefresh, trayMode, onTrayModeChange }: QuickMenuProps) {
  const { t } = useI18n();
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    menuRef.current?.querySelector<HTMLButtonElement>("button")?.focus();
  }, []);

  return (
    <div className="quick-menu" ref={menuRef} role="menu" aria-label={t("menu.label")}>
      {items.map((item) => (
        <button
          className={item.separator ? "quick-menu__item quick-menu__item--separator" : "quick-menu__item"}
          key={item.key}
          type="button"
          role="menuitem"
          onClick={() => {
            if (item.action === "refresh") onRefresh();
            onClose();
          }}
        >
          <Icon name={item.icon} />
          <span>{t(`menu.${item.key}`)}</span>
        </button>
      ))}
      <div className="quick-menu__tray-mode" role="group" aria-label={t("menu.trayValue")}>
        <span className="quick-menu__section-label">{t("menu.trayValue")}</span>
        {(["auto", "fiveHours", "weekly"] as const).map((mode) => (
          <button
            className="quick-menu__tray-option"
            key={mode}
            type="button"
            aria-pressed={trayMode === mode}
            onClick={() => onTrayModeChange(mode)}
          >
            <span aria-hidden="true">{trayMode === mode ? "✓" : ""}</span>
            <span>{t(`menu.tray.${mode}`)}</span>
          </button>
        ))}
      </div>
    </div>
  );
}
