import type { ClipboardEventItem, ClipboardReadFailureReason } from "../models/clipboard";

export const CLIPBOARD_EVENT_NAMES = {
  changed: "clipboard.changed",
  created: "clipboard.created",
  duplicated: "clipboard.duplicated",
  readFailed: "clipboard.read_failed",
  deleted: "clipboard.deleted",
  historyCleared: "history.cleared"
} as const;

export type ClipboardEventName = (typeof CLIPBOARD_EVENT_NAMES)[keyof typeof CLIPBOARD_EVENT_NAMES];

export interface ClipboardEventBasePayload {
  at: string;
}

export interface ClipboardCreatedPayload extends ClipboardEventBasePayload {
  item: ClipboardEventItem;
}

export interface ClipboardDuplicatedPayload extends ClipboardEventBasePayload {
  hash: string;
}

export interface ClipboardReadFailedPayload extends ClipboardEventBasePayload {
  reason: ClipboardReadFailureReason;
}

export interface ClipboardDeletedPayload extends ClipboardEventBasePayload {
  id: string;
}

export interface HistoryClearedPayload extends ClipboardEventBasePayload {
  deletedCount: number;
}
