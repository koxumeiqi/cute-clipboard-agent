import { describe, expect, it } from "vitest";
import {
  CLIPBOARD_COMMANDS,
  CLIPBOARD_EVENT_NAMES,
  DEFAULT_CLIPBOARD_RECORDING_SETTINGS,
  toClipboardEventItem
} from "./index";

describe("clipboard shared contracts", () => {
  it("keeps recording defaults local and enabled", () => {
    expect(DEFAULT_CLIPBOARD_RECORDING_SETTINGS).toEqual({
      paused: false,
      recordText: true,
      recordImage: true
    });
  });

  it("defines clipboard event names used by native services", () => {
    expect(CLIPBOARD_EVENT_NAMES.changed).toBe("clipboard.changed");
    expect(CLIPBOARD_EVENT_NAMES.created).toBe("clipboard.created");
    expect(CLIPBOARD_EVENT_NAMES.duplicated).toBe("clipboard.duplicated");
    expect(CLIPBOARD_EVENT_NAMES.readFailed).toBe("clipboard.read_failed");
    expect(CLIPBOARD_EVENT_NAMES.deleted).toBe("clipboard.deleted");
    expect(CLIPBOARD_EVENT_NAMES.historyCleared).toBe("history.cleared");
  });

  it("defines clipboard command names used by UI integrations", () => {
    expect(CLIPBOARD_COMMANDS.getClipboardRecordingSettings).toBe("get_clipboard_recording_settings");
    expect(CLIPBOARD_COMMANDS.updateClipboardRecordingSettings).toBe("update_clipboard_recording_settings");
    expect(CLIPBOARD_COMMANDS.setRecordingPaused).toBe("set_recording_paused");
    expect(CLIPBOARD_COMMANDS.suppressNextClipboardHash).toBe("suppress_next_clipboard_hash");
    expect(CLIPBOARD_COMMANDS.listClipboardHistory).toBe("list_clipboard_history");
    expect(CLIPBOARD_COMMANDS.getClipboardHistoryItem).toBe("get_clipboard_history_item");
    expect(CLIPBOARD_COMMANDS.deleteClipboardHistoryItem).toBe("delete_clipboard_history_item");
    expect(CLIPBOARD_COMMANDS.clearClipboardHistory).toBe("clear_clipboard_history");
    expect(CLIPBOARD_COMMANDS.updateClipboardHistorySettings).toBe("update_clipboard_history_settings");
  });

  it("removes complete text content from event items", () => {
    expect(
      toClipboardEventItem({
        id: "clip-1",
        type: "text",
        preview: "secret pre...",
        text: "secret preview should not be broadcast in full",
        hash: "hash-1",
        createdAt: "2026-06-10T00:00:00Z",
        updatedAt: "2026-06-10T00:00:00Z"
      })
    ).toEqual({
      id: "clip-1",
      type: "text",
      preview: "secret pre...",
      hash: "hash-1",
      createdAt: "2026-06-10T00:00:00Z",
      updatedAt: "2026-06-10T00:00:00Z"
    });
  });
});
