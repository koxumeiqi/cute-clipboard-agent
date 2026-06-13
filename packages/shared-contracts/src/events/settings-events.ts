import type { AppSettings } from "../models/settings";

export const SETTINGS_EVENT_NAMES = {
  updated: "settings.updated",
  clipboardRecordingPaused: "settings.clipboard_recording_paused",
  clipboardRecordingResumed: "settings.clipboard_recording_resumed"
} as const;

export type SettingsEventName = (typeof SETTINGS_EVENT_NAMES)[keyof typeof SETTINGS_EVENT_NAMES];

export interface SettingsUpdatedPayload {
  at: string;
  settings: AppSettings;
}

export interface SettingsRecordingStatePayload {
  at: string;
}
