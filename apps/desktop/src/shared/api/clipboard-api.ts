import { invoke } from "@tauri-apps/api/core";
import type {
  ClipboardItem,
  ClipboardHistoryItemRequest,
  ClipboardHistorySnapshot,
  ClipboardRecordingSettings,
  SetClipboardRecordingPausedRequest,
  SuppressNextClipboardHashRequest,
  UpdateClipboardHistorySettingsRequest,
  UpdateClipboardRecordingSettingsRequest
} from "@cute-clipboard/shared-contracts";
import { CLIPBOARD_COMMANDS } from "@cute-clipboard/shared-contracts";

export async function getClipboardRecordingSettings(): Promise<ClipboardRecordingSettings> {
  return invoke<ClipboardRecordingSettings>(CLIPBOARD_COMMANDS.getClipboardRecordingSettings);
}

export async function updateClipboardRecordingSettings(
  input: UpdateClipboardRecordingSettingsRequest
): Promise<ClipboardRecordingSettings> {
  return invoke<ClipboardRecordingSettings>(CLIPBOARD_COMMANDS.updateClipboardRecordingSettings, { input });
}

export async function setClipboardRecordingPaused(
  request: SetClipboardRecordingPausedRequest
): Promise<ClipboardRecordingSettings> {
  return invoke<ClipboardRecordingSettings>(CLIPBOARD_COMMANDS.setRecordingPaused, { request });
}

export async function suppressNextClipboardHash(request: SuppressNextClipboardHashRequest): Promise<void> {
  await invoke(CLIPBOARD_COMMANDS.suppressNextClipboardHash, { request });
}

export async function debugProcessClipboardText(text: string): Promise<ClipboardItem | null> {
  return invoke<ClipboardItem | null>("debug_process_clipboard_text", { request: { text } });
}

export async function listClipboardHistory(): Promise<ClipboardHistorySnapshot> {
  return invoke<ClipboardHistorySnapshot>(CLIPBOARD_COMMANDS.listClipboardHistory);
}

export async function getClipboardHistoryItem(request: ClipboardHistoryItemRequest): Promise<ClipboardItem> {
  return invoke<ClipboardItem>(CLIPBOARD_COMMANDS.getClipboardHistoryItem, { request });
}

export async function restoreClipboardHistoryItem(request: ClipboardHistoryItemRequest): Promise<void> {
  await invoke(CLIPBOARD_COMMANDS.restoreClipboardHistoryItem, { request });
}

export async function deleteClipboardHistoryItem(request: ClipboardHistoryItemRequest): Promise<ClipboardHistorySnapshot> {
  return invoke<ClipboardHistorySnapshot>(CLIPBOARD_COMMANDS.deleteClipboardHistoryItem, { request });
}

export async function clearClipboardHistory(): Promise<number> {
  return invoke<number>(CLIPBOARD_COMMANDS.clearClipboardHistory);
}

export async function updateClipboardHistorySettings(
  request: UpdateClipboardHistorySettingsRequest
): Promise<ClipboardHistorySnapshot> {
  return invoke<ClipboardHistorySnapshot>(CLIPBOARD_COMMANDS.updateClipboardHistorySettings, { request });
}
