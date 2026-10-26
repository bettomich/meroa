import { useEffect, useRef, useState } from "react";
import { useI18n } from "../i18n/I18nProvider";
import { openUpdateDownload, type UpdateState } from "../updates";

interface Props {
  state: UpdateState | null;
  unavailable: boolean;
  onCheck: () => Promise<void>;
  onAutomaticChange: (enabled: boolean) => Promise<void>;
}

export function UpdateSettings({ state, unavailable, onCheck, onAutomaticChange }: Props) {
  const { language, t } = useI18n();
  const [confirmationOpen, setConfirmationOpen] = useState(false);
  const [opening, setOpening] = useState(false);
  const [downloadError, setDownloadError] = useState(false);
  const [cooldownNotice, setCooldownNotice] = useState(false);
  const confirmRef = useRef<HTMLButtonElement>(null);
  const downloadRef = useRef<HTMLButtonElement>(null);
  const version = state?.available?.version;
  useEffect(() => {
    setConfirmationOpen(false);
    setDownloadError(false);
    setCooldownNotice(false);
  }, [version]);
  useEffect(() => { if (confirmationOpen) confirmRef.current?.focus(); }, [confirmationOpen]);

  const cancel = () => { setConfirmationOpen(false); downloadRef.current?.focus(); };
  const checkNow = () => {
    const elapsed = state?.lastAttemptAt == null ? null : Date.now() / 1000 - state.lastAttemptAt;
    const recent = elapsed !== null && elapsed >= 0 && elapsed < 60;
    setCooldownNotice(recent);
    if (!recent) void onCheck();
  };
  const download = async () => {
    if (!version || opening) return;
    setOpening(true);
    setDownloadError(false);
    try {
      await openUpdateDownload(version);
      cancel();
    } catch { setDownloadError(true); }
    finally { setOpening(false); }
  };

  const status = unavailable ? "unavailable" : state?.checking ? "checking"
    : version ? "available" : state?.error ? "error" : state?.checkedAt ? "upToDate" : "waiting";
  const date = (timestamp: number) => new Intl.DateTimeFormat(language === "it" ? "it-IT" : "en-GB", {
    day: "2-digit", month: "short", year: "numeric",
  }).format(new Date(timestamp * 1000));

  return <div className="page-section update-settings" aria-labelledby="updates-heading">
    <div id="updates-heading" className="page-section__label">{t("updates.title")}</div>
    <div className="settings-row">
      <span id="automatic-updates-label">{t("updates.automatic")}</span>
      <button type="button" className="settings-row__value settings-switch" role="switch" aria-labelledby="automatic-updates-label" aria-describedby="update-schedule" aria-checked={state?.automaticChecks ?? true} disabled={!state || unavailable} onClick={() => void onAutomaticChange(!state?.automaticChecks)}>
        <span className="settings-switch__track" aria-hidden="true"><span className="settings-switch__thumb" /></span>
        {state?.automaticChecks === false ? "OFF" : "ON"}
      </button>
    </div>
    <p id="update-schedule" className="settings-note">{t(state?.automaticChecks === false ? "updates.manualOnly" : "updates.schedule")}</p>
    <div className="update-settings__status" role="status" aria-live="polite">
      {t(`updates.${status}`)}{version ? ` · ${version}` : ""}
    </div>
    {state?.currentVersion ? <div className="settings-note">{t("updates.installed")} {state.currentVersion}</div> : null}
    {state?.checkedAt ? <div className="settings-note">{t("updates.lastChecked")} {date(state.checkedAt)}</div> : null}
    {version && state?.error ? <p className="settings-note">{t("updates.cached")}</p> : null}
    <div className="update-settings__actions">
      <button type="button" className="settings-row__value" disabled={!state || unavailable || state.checking} onClick={checkNow}>{t(state?.checking ? "updates.checking" : "updates.checkNow")}</button>
      {version ? <button type="button" className="settings-row__value" ref={downloadRef} onClick={() => setConfirmationOpen(true)}>{t("updates.download")}</button> : null}
    </div>
    {cooldownNotice ? <p className="settings-note" role="status">{t("updates.cooldown")}</p> : null}
    {confirmationOpen && version ? <div className="update-confirmation" role="group" aria-labelledby="update-confirmation-title" onKeyDown={(event) => {
      if (event.key === "Escape") { event.stopPropagation(); cancel(); }
    }}>
      <p id="update-confirmation-title">{t("updates.confirmTitle")} {version}?</p>
      <p className="settings-note">{t("updates.confirmDescription")}</p>
      <div className="update-settings__actions">
        <button type="button" className="settings-row__value" ref={confirmRef} disabled={opening} onClick={() => void download()}>{t("updates.confirmDownload")}</button>
        <button type="button" className="settings-row__value" disabled={opening} onClick={cancel}>{t("updates.later")}</button>
      </div>
      {downloadError ? <p className="settings-note" role="alert">{t("updates.downloadError")}</p> : null}
    </div> : null}
  </div>;
}
