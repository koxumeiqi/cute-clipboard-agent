import { invoke } from "@tauri-apps/api/core";
import type { AppSettings, UpdateAppSettingsRequest } from "@cute-clipboard/shared-contracts";
import { SETTINGS_COMMANDS } from "@cute-clipboard/shared-contracts";

export async function getAppSettings(): Promise<AppSettings> {
  return invoke<AppSettings>(SETTINGS_COMMANDS.getAppSettings);
}

export async function updateAppSettings(request: UpdateAppSettingsRequest): Promise<AppSettings> {
  return invoke<AppSettings>(SETTINGS_COMMANDS.updateAppSettings, { request });
}
