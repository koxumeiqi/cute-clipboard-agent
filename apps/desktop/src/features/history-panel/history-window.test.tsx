import { cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { ClipboardHistorySnapshot } from "@cute-clipboard/shared-contracts";
import { HistoryWindow } from "./history-window";

const clipboardApi = vi.hoisted(() => ({
  clearClipboardHistory: vi.fn(),
  deleteClipboardHistoryItem: vi.fn(),
  listClipboardHistory: vi.fn(),
  restoreClipboardHistoryItem: vi.fn()
}));

const petApi = vi.hoisted(() => ({
  closeHistoryPanel: vi.fn()
}));

const windowApi = vi.hoisted(() => ({
  hide: vi.fn()
}));

vi.mock("../../shared/api/clipboard-api", () => clipboardApi);
vi.mock("../../shared/api/pet-api", () => petApi);
vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => windowApi
}));

afterEach(() => {
  cleanup();
  vi.clearAllMocks();
});

describe("HistoryWindow", () => {
  it("renders text, emoji and image history entries", async () => {
    clipboardApi.listClipboardHistory.mockResolvedValue(historySnapshot());

    render(<HistoryWindow />);

    expect(await screen.findByRole("heading", { name: "剪贴板历史" })).toBeInTheDocument();
    expect(screen.getByText("最近 3 条记录")).toBeInTheDocument();
    expect(screen.getByText("会议纪要")).toBeInTheDocument();
    expect(screen.getByText("😀🚀")).toBeInTheDocument();
    expect(screen.getByRole("img", { name: "截图 1280x720" })).toHaveAttribute("src", "asset://thumb.png");
  });

  it("restores an image item when the item body is clicked", async () => {
    clipboardApi.listClipboardHistory.mockResolvedValue(historySnapshot());
    clipboardApi.restoreClipboardHistoryItem.mockResolvedValue(undefined);
    const user = userEvent.setup();

    render(<HistoryWindow />);
    await user.click(await screen.findByRole("button", { name: /恢复 图片 截图 1280x720/ }));

    expect(clipboardApi.restoreClipboardHistoryItem).toHaveBeenCalledWith({ id: "image-1" });
    expect(await screen.findByText("已恢复")).toBeInTheDocument();
  });

  it("deletes a single item without restoring it", async () => {
    clipboardApi.listClipboardHistory.mockResolvedValue(historySnapshot());
    clipboardApi.deleteClipboardHistoryItem.mockResolvedValue({
      ...historySnapshot(),
      items: [historySnapshot().items[1]],
      total: 1
    });
    const user = userEvent.setup();

    render(<HistoryWindow />);
    await user.click(await screen.findByRole("button", { name: "删除 会议纪要" }));

    expect(clipboardApi.deleteClipboardHistoryItem).toHaveBeenCalledWith({ id: "text-1" });
    expect(clipboardApi.restoreClipboardHistoryItem).not.toHaveBeenCalled();
    await waitFor(() => expect(screen.queryByText("会议纪要")).not.toBeInTheDocument());
  });

  it("clears all entries and shows the empty state", async () => {
    clipboardApi.listClipboardHistory
      .mockResolvedValueOnce(historySnapshot())
      .mockResolvedValueOnce(emptySnapshot());
    clipboardApi.clearClipboardHistory.mockResolvedValue(3);
    const user = userEvent.setup();

    render(<HistoryWindow />);
    await user.click(await screen.findByRole("button", { name: "清空历史" }));

    expect(clipboardApi.clearClipboardHistory).toHaveBeenCalledTimes(1);
    expect(await screen.findByText("暂无剪贴板历史")).toBeInTheDocument();
  });

  it("renders empty state on first load", async () => {
    clipboardApi.listClipboardHistory.mockResolvedValue(emptySnapshot());

    render(<HistoryWindow />);

    expect(await screen.findByText("暂无剪贴板历史")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "清空历史" })).toBeDisabled();
  });

  it("closes the history window through Rust", async () => {
    clipboardApi.listClipboardHistory.mockResolvedValue(emptySnapshot());
    petApi.closeHistoryPanel.mockResolvedValue(undefined);
    const user = userEvent.setup();

    render(<HistoryWindow />);
    await user.click(await screen.findByRole("button", { name: "关闭历史窗口" }));

    await waitFor(() => expect(petApi.closeHistoryPanel).toHaveBeenCalledTimes(1));
    expect(windowApi.hide).not.toHaveBeenCalled();
  });
});

function historySnapshot(): ClipboardHistorySnapshot {
  return {
    settings: { capacity: 20, persistEnabled: true },
    total: 3,
    items: [
      {
        id: "text-1",
        type: "text",
        preview: "会议纪要",
        text: "会议纪要",
        hash: "hash-text-1",
        createdAt: "2026-06-10T08:00:00Z",
        updatedAt: "2026-06-10T08:00:00Z"
      },
      {
        id: "emoji-1",
        type: "text",
        preview: "😀🚀",
        text: "😀🚀",
        hash: "hash-emoji-1",
        createdAt: "2026-06-10T07:00:00Z",
        updatedAt: "2026-06-10T07:00:00Z"
      },
      {
        id: "image-1",
        type: "image",
        preview: "截图 1280x720",
        imagePath: "asset://image.png",
        thumbnailPath: "asset://thumb.png",
        hash: "hash-image-1",
        createdAt: "2026-06-10T06:00:00Z",
        updatedAt: "2026-06-10T06:00:00Z"
      }
    ]
  };
}

function emptySnapshot(): ClipboardHistorySnapshot {
  return {
    settings: { capacity: 20, persistEnabled: true },
    total: 0,
    items: []
  };
}
