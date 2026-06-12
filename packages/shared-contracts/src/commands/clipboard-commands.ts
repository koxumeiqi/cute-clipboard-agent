export const CLIPBOARD_COMMANDS = {
  getClipboardRecordingSettings: "get_clipboard_recording_settings",
  updateClipboardRecordingSettings: "update_clipboard_recording_settings",
  setRecordingPaused: "set_recording_paused",
  suppressNextClipboardHash: "suppress_next_clipboard_hash",
  listClipboardHistory: "list_clipboard_history",
  getClipboardHistoryItem: "get_clipboard_history_item",
  restoreClipboardHistoryItem: "restore_clipboard_history_item",
  deleteClipboardHistoryItem: "delete_clipboard_history_item",
  clearClipboardHistory: "clear_clipboard_history",
  updateClipboardHistorySettings: "update_clipboard_history_settings"
} as const;

export interface UpdateClipboardRecordingSettingsRequest {
  recordText?: boolean;
  recordImage?: boolean;
}

export interface SetClipboardRecordingPausedRequest {
  paused: boolean;
}

export interface SuppressNextClipboardHashRequest {
  hash: string;
}

export interface ClipboardHistorySettings {
  capacity: 10 | 20 | 50;
  persistEnabled: boolean;
}

export interface ClipboardHistorySnapshot {
  items: import("../models/clipboard").ClipboardItem[];
  settings: ClipboardHistorySettings;
  total: number;
}

export interface ClipboardHistoryItemRequest {
  id: string;
}

export interface UpdateClipboardHistorySettingsRequest {
  capacity?: 10 | 20 | 50;
  persistEnabled?: boolean;
}
