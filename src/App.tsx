import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { DottedGauge } from "./components/DottedGauge";
import { Icon } from "./components/Icon";
import { MetricRow } from "./components/MetricRow";
import { QuickMenu } from "./components/QuickMenu";
import { useI18n } from "./i18n/I18nProvider";
import { mockUsage } from "./mocks/usage";

export function App() {
  const { t } = useI18n();
  const [menuOpen, setMenuOpen] = useState(false);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const refreshTimer = useRef<number | undefined>(undefined);
  const stageRef = useRef<HTMLElement>(null);

  const closePopover = useCallback(async () => {
    setMenuOpen(false);
    if (isTauri()) await invoke("hide_popover");
  }, []);

  const refresh = useCallback(() => {
    if (isRefreshing) return;
    setIsRefreshing(true);
    window.clearTimeout(refreshTimer.current);
    refreshTimer.current = window.setTimeout(() => setIsRefreshing(false), 650);
  }, [isRefreshing]);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      if (menuOpen) setMenuOpen(false);
      else void closePopover();
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.clearTimeout(refreshTimer.current);
    };
  }, [closePopover, menuOpen]);

  useLayoutEffect(() => {
    if (!isTauri()) return;

    const stage = stageRef.current;
    if (!stage) return;

    let animationFrame = 0;
    let lastSize = "";
    const syncGeometry = () => {
      window.cancelAnimationFrame(animationFrame);
      animationFrame = window.requestAnimationFrame(() => {
        const bounds = stage.getBoundingClientRect();
        const width = Math.ceil(bounds.width);
        const height = Math.ceil(bounds.height);
        const nextSize = `${width}x${height}`;
        if (nextSize === lastSize) return;
        lastSize = nextSize;
        void invoke<number>("sync_popover_geometry", { width, height }).then((pointerX) => {
          stage.style.setProperty("--pointer-x", `${pointerX}px`);
        });
      });
    };

    const observer = new ResizeObserver(syncGeometry);
    observer.observe(stage);
    void document.fonts.ready.then(syncGeometry);
    syncGeometry();

    return () => {
      observer.disconnect();
      window.cancelAnimationFrame(animationFrame);
    };
  }, []);

  useEffect(() => {
    if (!isTauri()) return;
    let unlisten: (() => void) | undefined;
    void listen<number>("popover-pointer", ({ payload }) => {
      stageRef.current?.style.setProperty("--pointer-x", `${payload}px`);
    }).then((stopListening) => {
      unlisten = stopListening;
    });
    return () => unlisten?.();
  }, []);

  return (
    <main className="app-stage" ref={stageRef}>
      <section className="popover" aria-label={t("app.popoverLabel")}>
        <header className="popover__header">
          <div className="identity">
            <div className="brand">MEROA</div>
            <div className="status" aria-label={t("status.safeDescription")}>
              <span className="status__dot" aria-hidden="true" />
              <span>{t("status.safe")}</span>
            </div>
          </div>
          <div className="header-actions">
            <button
              className="icon-button"
              type="button"
              aria-label={t("common.refresh")}
              onClick={refresh}
            >
              <Icon name="refresh" className={isRefreshing ? "is-spinning" : ""} />
            </button>
            <button
              className="icon-button"
              type="button"
              aria-label={t("common.more")}
              aria-expanded={menuOpen}
              onClick={() => setMenuOpen((open) => !open)}
            >
              <Icon name="more" />
            </button>
          </div>
        </header>

        <div className="hero-panel">
          <DottedGauge
            value={mockUsage.remaining}
            label={t("usage.remaining")}
          />
        </div>

        <div className="metrics" aria-label={t("usage.metricsLabel")}>
          <MetricRow icon="clock" label={t("usage.fiveHours")} value={`${mockUsage.fiveHours}%`} />
          <MetricRow icon="calendar" label={t("usage.weekly")} value={`${mockUsage.weekly}%`} />
          <MetricRow
            icon="cursor"
            label={t("usage.credits")}
            value={String(mockUsage.credits)}
          />
          <MetricRow icon="reset" label={t("usage.reset")} value={t("usage.resetValue")} />
        </div>

        <footer className="popover__footer">
          <div className="freshness">
            <span className="freshness__dot" aria-hidden="true" />
            <span>{t("usage.updatedNow")}</span>
          </div>
          <button className="icon-button" type="button" aria-label={t("common.refresh")} onClick={refresh}>
            <Icon name="refresh" className={isRefreshing ? "is-spinning" : ""} />
          </button>
        </footer>

        {menuOpen ? <QuickMenu onClose={() => setMenuOpen(false)} onRefresh={refresh} /> : null}
      </section>
      <span className="popover-pointer" aria-hidden="true" />
    </main>
  );
}
