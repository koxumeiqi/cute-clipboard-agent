import type { AppSettings, HistoryCapacity } from "../models/settings";

export const SETTINGS_COMMANDS = {
  getAppSettings: "get_app_settings",
  updateAppSettings: "update_app_settings"
} as const;

export interface UpdateAppSettingsRequest {
  historyCapacity?: HistoryCapacity;
  recordText?: boolean;
  recordImage?: boolean;
  idleAnimationEnabled?: boolean;
  autoMoveEnabled?: boolean;
  launchAtStartup?: boolean;
  persistenceEnabled?: boolean;
  recordingPaused?: boolean;
}

export type GetAppSettingsResponse = AppSettings;
export type UpdateAppSettingsResponse = AppSettings;
