import { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { loadAutostartState, readAutostartState, setAutostartEnabled, type AutostartState } from "./autostart";
import { DottedGauge } from "./components/DottedGauge";
import { Icon } from "./components/Icon";
import { MetricRow } from "./components/MetricRow";
import { QuickMenu } from "./components/QuickMenu";
import { useI18n } from "./i18n/I18nProvider";
import { requestNativeQuit } from "./quit";
import {
  formatCredits,
  formatReset,
  readHeroSelection,
  readTrayMode,
  relativeUpdatedLabel,
  saveHeroSelection,
  saveTrayMode,
  selectedHeroMetric,
  snapshotIsStale,
  usageData,
  type HeroSelection,
  type TrayMode,
  type UsageState,
} from "./usage";

export function App() {
  const { language, setLanguage, t } = useI18n();
  const [menuOpen, setMenuOpen] = useState(false);
  const [view, setView] = useState<"home" | "settings" | "about">("home");
  const [autostartState, setAutostartState] = useState<AutostartState>("loading");
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [usageState, setUsageState] = useState<UsageState>({ status: "loading" });
  const [heroSelection, setHeroSelection] = useState<HeroSelection>(readHeroSelection);
  const [trayMode, setTrayMode] = useState<TrayMode>(readTrayMode);
  const [nowSeconds, setNowSeconds] = useState(() => Date.now() / 1000);
  const stageRef = useRef<HTMLElement>(null);

  const closePopover = useCallback(async () => {
    setMenuOpen(false);
    setView("home");
    if (isTauri()) await invoke("hide_popover");
  }, []);

  const refresh = useCallback(async () => {
    if (isRefreshing) return;
    setIsRefreshing(true);
    try {
      if (isTauri()) {
        setUsageState(await invoke<UsageState>("refresh_usage"));
      }
    } finally {
      setIsRefreshing(false);
    }
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
    };
  }, [closePopover, menuOpen]);

  useEffect(() => {
    if (!isTauri()) {
      setUsageState({
        status: "unavailable",
        message: "Real Codex usage is available only in the native MEROA app.",
      });
      return;
    }

    let unlisten: (() => void) | undefined;
    void invoke<UsageState>("get_usage_state").then(setUsageState);
    void listen<UsageState>("usage-updated", ({ payload }) => setUsageState(payload)).then(
      (stopListening) => {
        unlisten = stopListening;
      },
    );
    return () => unlisten?.();
  }, []);

  useEffect(() => {
    const timer = window.setInterval(() => setNowSeconds(Date.now() / 1000), 15_000);
    return () => window.clearInterval(timer);
  }, []);

  useEffect(() => {
    if (isTauri()) void invoke("set_tray_language", { language });
  }, [language]);

  useEffect(() => {
    if (!isTauri()) {
      setAutostartState("disabled");
      return;
    }
    let active = true;
    void loadAutostartState()
      .then((state) => {
        if (active) setAutostartState(state);
      })
      .catch(() => {
        if (active) setAutostartState("error");
      });
    return () => { active = false; };
  }, []);

  useEffect(() => {
    if (isTauri()) void invoke("set_tray_mode", { mode: trayMode });
  }, [trayMode]);

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

  const data = usageData(usageState);
  const heroMetric = useMemo(
    () => selectedHeroMetric(data, heroSelection, nowSeconds),
    [data, heroSelection, nowSeconds],
  );
  const isStale = data ? snapshotIsStale(data.fetchedAt, nowSeconds) : usageState.status === "stale";
  const heroLabel = data ? t("usage.remaining") : t(`state.${usageState.status}`);
  const stateMessage = "message" in usageState ? usageState.message : undefined;
  const freshnessText = data
    ? isStale ? t("state.stale") : relativeUpdatedLabel(data.fetchedAt, language, nowSeconds)
    : t(`state.${usageState.status}`);
  const statusText = data && !isStale ? t("state.live") : t(`state.${isStale ? "stale" : usageState.status}`);
  const statusTone = data && !isStale
    ? "live"
    : isStale
      ? "stale"
      : usageState.status;
  const selectHero = (selection: HeroSelection) => {
    setHeroSelection(selection);
    saveHeroSelection(selection);
  };
  const selectTrayMode = (mode: TrayMode) => {
    setTrayMode(mode);
    saveTrayMode(mode);
  };

  const toggleAutostart = () => {
    if (!isTauri() || autostartState === "loading") return;
    setAutostartState("loading");
    void readAutostartState()
      .then((currentState) => setAutostartEnabled(currentState !== "enabled"))
      .then(setAutostartState)
      .catch(() => setAutostartState("error"));
  };

  const openView = (nextView: "settings" | "about") => {
    setView(nextView);
    setMenuOpen(false);
  };

  const quitApplication = async () => {
    if (!isTauri()) return;
    try {
      await requestNativeQuit(invoke);
    } catch {
      setUsageState({ status: "error", message: t("state.quitFailed") });
    }
  };

  return (
    <main className="app-stage" ref={stageRef}>
      <section className="popover" aria-label={t("app.popoverLabel")}>
        {view === "home" ? <header className="popover__header">
          <div className="identity">
            <div className="brand">MEROA</div>
            <div className={`status status--${statusTone}`} aria-label={stateMessage ?? statusText}>
              <span className="status__dot" aria-hidden="true" />
              <span>{statusText}</span>
            </div>
          </div>
          <div className="header-actions">
            <button
              className="icon-button"
              type="button"
              aria-label={t("common.refresh")}
              onClick={() => void refresh()}
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
        </header> : <header className="page-header">
          <button className="page-header__back" type="button" onClick={() => setView("home")}>
            <span aria-hidden="true">←</span>
            <span>{t(`page.${view}.title`)}</span>
          </button>
        </header>}

        {view === "home" ? <>
        <div className="hero-panel">
          <DottedGauge
            value={heroMetric.percentage}
            displayValue={heroMetric.value}
            label={data ? t(`usage.${heroMetric.label}`) : heroLabel}
            valueKind={heroMetric.label === "reset" ? "duration" : "default"}
          />
        </div>

        <div className="metrics" aria-label={t("usage.metricsLabel")}>
          <MetricRow
            icon="clock"
            label={t("usage.fiveHours")}
            value={data?.fiveHours ? `${data.fiveHours.remainingPercent}%` : "—"}
            selected={heroSelection === "fiveHours"}
            onSelect={() => selectHero("fiveHours")}
          />
          <MetricRow
            icon="calendar"
            label={t("usage.weekly")}
            value={data?.weekly ? `${data.weekly.remainingPercent}%` : "—"}
            selected={heroSelection === "weekly"}
            onSelect={() => selectHero("weekly")}
          />
          <MetricRow
            icon="cursor"
            label={t("usage.credits")}
            value={formatCredits(data?.credits ?? null)}
            selected={heroSelection === "credits"}
            onSelect={() => selectHero("credits")}
          />
          <MetricRow
            icon="reset"
            label={t("usage.reset")}
            value={formatReset(data?.limitingWindow.resetsAt ?? null)}
            selected={heroSelection === "reset"}
            onSelect={() => selectHero("reset")}
          />
        </div>

        <footer className="popover__footer">
          <div className={`freshness freshness--${statusTone}`} title={stateMessage}>
            <span className="freshness__dot" aria-hidden="true" />
            <span>{freshnessText}</span>
          </div>
        </footer>
        </> : view === "settings" ? <section className="page-view settings-view" aria-label={t("page.settings.title")}>
          <div className="page-section">
            <div className="page-section__label">{t("page.settings.general")}</div>
            <div className="settings-row"><span>{t("page.settings.language")}</span><button type="button" className="settings-row__value" onClick={() => setLanguage(language === "it" ? "en" : "it")}>{language === "it" ? t("page.settings.italian") : t("page.settings.english")} <span aria-hidden="true">›</span></button></div>
            <div className="settings-row"><span>{t("page.settings.autostart")}</span><button type="button" className="settings-row__value settings-row__value--muted" aria-pressed={autostartState === "enabled"} disabled={autostartState === "loading"} onClick={toggleAutostart}>{t(`page.settings.autostartState.${autostartState}`)}</button></div>
          </div>
          <div className="page-section">
            <div className="page-section__label">{t("page.settings.monitoring")}</div>
            <div className="settings-row"><span>{t("page.settings.trayValue")}</span><span className="settings-row__value">{t(`menu.tray.${trayMode}`)} <span aria-hidden="true">›</span></span></div>
            <div className="settings-row"><span>{t("page.settings.refresh")}</span><span className="settings-row__value settings-row__value--muted">{t("page.settings.everyFiveMinutes")}</span></div>
          </div>
          <div className="page-section">
            <div className="page-section__label">{t("page.settings.data")}</div>
            <div className="settings-row"><span>{t("page.settings.storage")}</span><span className="settings-row__value settings-row__value--muted">{t("page.settings.deviceOnly")}</span></div>
          </div>
        </section> : <section className="page-view about-view" aria-label={t("page.about.title")}>
          <div className="about-view__brand">MEROA</div>
          <div className="about-view__version">{t("page.about.currentVersion")}</div>
          <p className="about-view__tagline">{t("page.about.tagline")}</p>
          <div className="about-view__local"><span className="status__dot" aria-hidden="true" />{t("page.about.localFirst")}</div>
          <div className="about-view__muted">{t("page.about.noCloud")}</div>
        </section>}

        {menuOpen ? <QuickMenu onClose={() => setMenuOpen(false)} onNavigate={openView} onQuit={quitApplication} trayMode={trayMode} onTrayModeChange={selectTrayMode} /> : null}
      </section>
      <span className="popover-pointer" aria-hidden="true" />
    </main>
  );
}
