import { useEffect, useState } from "react";
import { Check, Image, PauseCircle, Power, Rabbit, Save, Type, X } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type {
  AppSettings,
  HistoryCapacity,
  UpdateAppSettingsRequest
} from "@cute-clipboard/shared-contracts";
import { getAppSettings, updateAppSettings } from "../../shared/api/settings-api";

type LoadState = "loading" | "ready" | "failed";
type SaveState = "idle" | "saving" | "saved" | "failed";

const capacityOptions: HistoryCapacity[] = [10, 20, 50];
const text = {
  settings: "\u8bbe\u7f6e",
  subtitle: "\u672c\u5730\u8bb0\u5f55\u548c\u684c\u5ba0\u504f\u597d",
  close: "\u5173\u95ed\u8bbe\u7f6e\u7a97\u53e3",
  loading: "\u6b63\u5728\u8bfb\u53d6\u8bbe\u7f6e...",
  loadFailed: "\u8bbe\u7f6e\u8bfb\u53d6\u5931\u8d25\uff0c\u8bf7\u7a0d\u540e\u91cd\u8bd5\u3002",
  history: "\u5386\u53f2\u8bb0\u5f55",
  capacity: "\u5386\u53f2\u5bb9\u91cf",
  capacityDescription: "\u8d85\u8fc7\u5bb9\u91cf\u540e\u81ea\u52a8\u4fdd\u7559\u6700\u65b0\u8bb0\u5f55",
  capacityUnit: "\u6761",
  recordText: "\u8bb0\u5f55\u6587\u672c\u548c emoji",
  recordTextDescription: "\u5173\u95ed\u540e\u6587\u672c\u548c emoji \u4e0d\u4f1a\u8fdb\u5165\u5386\u53f2",
  recordImage: "\u8bb0\u5f55\u56fe\u7247",
  recordImageDescription: "\u5173\u95ed\u540e\u56fe\u7247\u4e0d\u4f1a\u8fdb\u5165\u5386\u53f2",
  pauseRecording: "\u6682\u505c\u8bb0\u5f55",
  pauseRecordingDescription: "\u5f00\u542f\u540e\u4efb\u4f55\u65b0\u526a\u8d34\u677f\u5185\u5bb9\u90fd\u4e0d\u4f1a\u8fdb\u5165\u5386\u53f2",
  petBehavior: "\u684c\u5ba0\u884c\u4e3a",
  idleAnimation: "\u5f85\u673a\u52a8\u753b",
  idleAnimationDescription: "\u5173\u95ed\u540e\u684c\u5ba0\u4fdd\u6301\u9759\u6b62",
  autoMove: "\u81ea\u52a8\u79fb\u52a8",
  autoMoveDescription: "\u5141\u8bb8\u540e\u7eed\u684c\u5ba0\u884c\u4e3a\u4f7f\u7528\u81ea\u52a8\u79fb\u52a8\u504f\u597d",
  startupAndLocal: "\u542f\u52a8\u4e0e\u672c\u5730\u4fdd\u5b58",
  launchAtStartup: "\u5f00\u673a\u542f\u52a8",
  launchAtStartupDescription: "\u4fdd\u5b58\u542f\u52a8\u504f\u597d\uff0c\u5e95\u5c42\u6ce8\u518c\u7531\u751f\u547d\u5468\u671f\u6a21\u5757\u63a5\u5165",
  persistence: "\u672c\u5730\u6301\u4e45\u5316",
  persistenceDescription: "\u5173\u95ed\u540e\u65b0\u5386\u53f2\u53ea\u4fdd\u7559\u5728\u5f53\u524d\u8fd0\u884c\u5185\u5b58\u4e2d",
  saving: "\u6b63\u5728\u4fdd\u5b58...",
  saved: "\u5df2\u4fdd\u5b58",
  saveFailed: "\u8bbe\u7f6e\u4fdd\u5b58\u5931\u8d25\uff0c\u8bf7\u7a0d\u540e\u91cd\u8bd5\u3002"
} as const;

export function SettingsWindow() {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [saveState, setSaveState] = useState<SaveState>("idle");

  useEffect(() => {
    let cancelled = false;
    getAppSettings()
      .then((nextSettings) => {
        if (!cancelled) {
          setSettings(nextSettings);
          setLoadState("ready");
        }
      })
      .catch(() => {
        if (!cancelled) setLoadState("failed");
      });
    return () => {
      cancelled = true;
    };
  }, []);

  const save = async (request: UpdateAppSettingsRequest) => {
    if (!settings) return;
    const previous = settings;
    setSaveState("saving");
    setSettings({ ...settings, ...request });
    try {
      const nextSettings = await updateAppSettings(request);
      setSettings(nextSettings);
      setSaveState("saved");
      window.setTimeout(() => setSaveState("idle"), 1400);
    } catch {
      setSettings(previous);
      setSaveState("failed");
    }
  };

  const disabled = saveState === "saving" || loadState !== "ready";

  return (
    <section className="settings-window" aria-label={text.settings}>
      <header className="settings-header">
        <div>
          <h1>{text.settings}</h1>
          <p>{text.subtitle}</p>
        </div>
        <button type="button" className="icon-button" aria-label={text.close} onClick={() => void getCurrentWindow().close()}>
          <X size={18} aria-hidden="true" />
        </button>
      </header>

      <main className="settings-content">
        {loadState === "loading" ? <p className="settings-status">{text.loading}</p> : null}
        {loadState === "failed" ? <p className="settings-status settings-status--error">{text.loadFailed}</p> : null}
        {loadState === "ready" && settings ? (
          <>
            <section className="settings-section" aria-labelledby="history-settings-title">
              <h2 id="history-settings-title">{text.history}</h2>
              <div className="setting-row">
                <div>
                  <span className="setting-label">{text.capacity}</span>
                  <span className="setting-description">{text.capacityDescription}</span>
                </div>
                <div className="segmented-control" aria-label={text.capacity}>
                  {capacityOptions.map((capacity) => (
                    <button
                      key={capacity}
                      type="button"
                      aria-pressed={settings.historyCapacity === capacity}
                      disabled={disabled}
                      onClick={() => void save({ historyCapacity: capacity })}
                    >
                      {capacity} {text.capacityUnit}
                    </button>
                  ))}
                </div>
              </div>
              <ToggleRow
                icon={<Type size={18} aria-hidden="true" />}
                label={text.recordText}
                description={text.recordTextDescription}
                checked={settings.recordText}
                disabled={disabled}
                onChange={(recordText) => void save({ recordText })}
              />
              <ToggleRow
                icon={<Image size={18} aria-hidden="true" />}
                label={text.recordImage}
                description={text.recordImageDescription}
                checked={settings.recordImage}
                disabled={disabled}
                onChange={(recordImage) => void save({ recordImage })}
              />
              <ToggleRow
                icon={<PauseCircle size={18} aria-hidden="true" />}
                label={text.pauseRecording}
                description={text.pauseRecordingDescription}
                checked={settings.recordingPaused}
                disabled={disabled}
                onChange={(recordingPaused) => void save({ recordingPaused })}
              />
            </section>

            <section className="settings-section" aria-labelledby="pet-settings-title">
              <h2 id="pet-settings-title">{text.petBehavior}</h2>
              <ToggleRow
                icon={<Rabbit size={18} aria-hidden="true" />}
                label={text.idleAnimation}
                description={text.idleAnimationDescription}
                checked={settings.idleAnimationEnabled}
                disabled={disabled}
                onChange={(idleAnimationEnabled) => void save({ idleAnimationEnabled })}
              />
              <ToggleRow
                icon={<Rabbit size={18} aria-hidden="true" />}
                label={text.autoMove}
                description={text.autoMoveDescription}
                checked={settings.autoMoveEnabled}
                disabled={disabled}
                onChange={(autoMoveEnabled) => void save({ autoMoveEnabled })}
              />
            </section>

            <section className="settings-section" aria-labelledby="privacy-settings-title">
              <h2 id="privacy-settings-title">{text.startupAndLocal}</h2>
              <ToggleRow
                icon={<Power size={18} aria-hidden="true" />}
                label={text.launchAtStartup}
                description={text.launchAtStartupDescription}
                checked={settings.launchAtStartup}
                disabled={disabled}
                onChange={(launchAtStartup) => void save({ launchAtStartup })}
              />
              <ToggleRow
                icon={<Save size={18} aria-hidden="true" />}
                label={text.persistence}
                description={text.persistenceDescription}
                checked={settings.persistenceEnabled}
                disabled={disabled}
                onChange={(persistenceEnabled) => void save({ persistenceEnabled })}
              />
            </section>
          </>
        ) : null}
      </main>

      <footer className="settings-footer" aria-live="polite">
        {saveState === "saving" ? <span>{text.saving}</span> : null}
        {saveState === "saved" ? (
          <span className="settings-saved">
            <Check size={15} aria-hidden="true" />
            {text.saved}
          </span>
        ) : null}
        {saveState === "failed" ? <span className="settings-error">{text.saveFailed}</span> : null}
      </footer>
    </section>
  );
}

interface ToggleRowProps {
  icon: React.ReactNode;
  label: string;
  description: string;
  checked: boolean;
  disabled: boolean;
  onChange: (checked: boolean) => void;
}

function ToggleRow({ icon, label, description, checked, disabled, onChange }: ToggleRowProps) {
  return (
    <div className="setting-row">
      <div className="setting-copy">
        <span className="setting-icon">{icon}</span>
        <span>
          <span className="setting-label">{label}</span>
          <span className="setting-description">{description}</span>
        </span>
      </div>
      <button
        type="button"
        role="switch"
        aria-checked={checked}
        aria-label={label}
        className="switch-button"
        disabled={disabled}
        onClick={() => onChange(!checked)}
      >
        <span aria-hidden="true" />
      </button>
    </div>
  );
}
