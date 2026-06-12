export type ClipboardItemType = "text" | "image";

export interface ClipboardItem {
  id: string;
  type: ClipboardItemType;
  preview: string;
  text?: string;
  imagePath?: string;
  thumbnailPath?: string;
  hash: string;
  createdAt: string;
  updatedAt: string;
}

export interface ClipboardEventItem {
  id: string;
  type: ClipboardItemType;
  preview: string;
  imagePath?: string;
  thumbnailPath?: string;
  hash: string;
  createdAt: string;
  updatedAt: string;
}

export interface ClipboardRecordingSettings {
  paused: boolean;
  recordText: boolean;
  recordImage: boolean;
}

export const DEFAULT_CLIPBOARD_RECORDING_SETTINGS: ClipboardRecordingSettings = {
  paused: false,
  recordText: true,
  recordImage: true
};

export type ClipboardReadFailureReason =
  | "empty"
  | "unsupportedType"
  | "readFailed"
  | "recordingPaused"
  | "typeDisabled"
  | "duplicate"
  | "selfWriteSuppressed";

export function toClipboardEventItem(item: ClipboardItem): ClipboardEventItem {
  const { text: _text, ...eventItem } = item;
  return eventItem;
}
