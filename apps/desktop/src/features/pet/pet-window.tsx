import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { PetSettings, SettingsUpdatedPayload } from "@cute-clipboard/shared-contracts";
import { DEFAULT_PET_SETTINGS, SETTINGS_EVENT_NAMES } from "@cute-clipboard/shared-contracts";
import { currentWindowOuterPosition, getPetSettings, movePetWindowBy, openHistoryPanel, openSettingsWindow, quitApp, savePetPosition, setRecordingPaused } from "../../shared/api/pet-api";
import { Pet } from "./pet";

export function PetWindow() {
  const [settings, setSettings] = useState<PetSettings>(DEFAULT_PET_SETTINGS);
  const [loading, setLoading] = useState(true);
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    let cancelled = false;
    getPetSettings()
      .then((nextSettings) => {
        if (!cancelled) setSettings(nextSettings);
      })
      .catch(() => {
        if (!cancelled) setFailed(true);
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });

    const fallbackTimer = window.setTimeout(() => {
      if (!cancelled) setLoading(false);
    }, 1200);

    return () => {
      cancelled = true;
      window.clearTimeout(fallbackTimer);
    };
  }, []);

  useEffect(() => {
    let disposed = false;
    let unlisten: (() => void) | undefined;
    listen<SettingsUpdatedPayload>(SETTINGS_EVENT_NAMES.updated, ({ payload }) => {
      if (disposed) return;
      setSettings((current) => ({
        ...current,
        idleAnimationEnabled: payload.settings.idleAnimationEnabled,
        autoMoveEnabled: payload.settings.autoMoveEnabled
      }));
    })
      .then((nextUnlisten) => {
        if (disposed) {
          nextUnlisten();
        } else {
          unlisten = nextUnlisten;
        }
      })
      .catch(() => {
        if (!disposed) setFailed(true);
      });
    return () => {
      disposed = true;
      unlisten?.();
    };
  }, []);

  useEffect(() => {
    let disposed = false;
    let unlisten: (() => void) | undefined;

    getCurrentWindow()
      .onMoved(async ({ payload }) => {
        if (disposed) return;
        try {
          const nextSettings = await savePetPosition(payload);
          if (!disposed) setSettings(nextSettings);
        } catch {
          if (!disposed) setFailed(true);
        }
      })
      .then((nextUnlisten) => {
        if (disposed) {
          nextUnlisten();
        } else {
          unlisten = nextUnlisten;
        }
      })
      .catch(() => {
        if (!disposed) setFailed(true);
      });

    return () => {
      disposed = true;
      unlisten?.();
    };
  }, []);

  const handleDragEnd = async () => {
    try {
      const position = await currentWindowOuterPosition();
      const nextSettings = await savePetPosition(position);
      setSettings(nextSettings);
    } catch {
      setFailed(true);
    }
  };

  const handleDragMove = async (deltaX: number, deltaY: number) => {
    try {
      await movePetWindowBy({ deltaX, deltaY });
    } catch {
      setFailed(true);
    }
  };

  return (
    <Pet
      settings={settings}
      disabled={false}
      onDragMove={handleDragMove}
      onDragEnd={handleDragEnd}
      onOpenHistory={() => void openHistoryPanel().catch(() => setFailed(true))}
      onOpenSettings={() => void openSettingsWindow().catch(() => setFailed(true))}
      onPauseRecording={() => void setRecordingPaused(true).catch(() => setFailed(true))}
      onQuit={() => void quitApp().catch(() => setFailed(true))}
    />
  );
}
