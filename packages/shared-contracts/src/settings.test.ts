import { describe, expect, it } from "vitest";
import { DEFAULT_APP_SETTINGS, SETTINGS_COMMANDS, SETTINGS_EVENT_NAMES, isHistoryCapacity } from "./index";

describe("settings contracts", () => {
  it("defines local-first default app settings", () => {
    expect(DEFAULT_APP_SETTINGS).toEqual({
      historyCapacity: 20,
      recordText: true,
      recordImage: true,
      idleAnimationEnabled: true,
      autoMoveEnabled: false,
      launchAtStartup: false,
      persistenceEnabled: true,
      recordingPaused: false
    });
  });

  it("keeps settings command names stable", () => {
    expect(SETTINGS_COMMANDS).toEqual({
      getAppSettings: "get_app_settings",
      updateAppSettings: "update_app_settings"
    });
  });

  it("keeps settings event names stable", () => {
    expect(SETTINGS_EVENT_NAMES).toEqual({
      updated: "settings.updated",
      clipboardRecordingPaused: "settings.clipboard_recording_paused",
      clipboardRecordingResumed: "settings.clipboard_recording_resumed"
    });
  });

  it("accepts only PRD history capacities", () => {
    expect(isHistoryCapacity(10)).toBe(true);
    expect(isHistoryCapacity(20)).toBe(true);
    expect(isHistoryCapacity(50)).toBe(true);
    expect(isHistoryCapacity(11)).toBe(false);
  });
});
