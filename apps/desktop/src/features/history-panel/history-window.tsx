import { useEffect, useState } from "react";
import { Check, Image as ImageIcon, RotateCcw, Trash2, X } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { ClipboardHistorySnapshot, ClipboardItem } from "@cute-clipboard/shared-contracts";
import {
  clearClipboardHistory,
  deleteClipboardHistoryItem,
  listClipboardHistory,
  restoreClipboardHistoryItem
} from "../../shared/api/clipboard-api";
import { closeHistoryPanel } from "../../shared/api/pet-api";

type LoadState = "loading" | "ready" | "failed";

export function HistoryWindow() {
  const [snapshot, setSnapshot] = useState<ClipboardHistorySnapshot | null>(null);
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [restoredId, setRestoredId] = useState<string | null>(null);
  const [busyItemId, setBusyItemId] = useState<string | null>(null);

  const refresh = async () => {
    setLoadState("loading");
    try {
      setSnapshot(await listClipboardHistory());
      setLoadState("ready");
    } catch {
      setLoadState("failed");
    }
  };

  useEffect(() => {
    void refresh();
  }, []);

  const closeWindow = () => {
    void closeHistoryPanel().catch(() => {
      void getCurrentWindow().hide();
    });
  };

  const restoreItem = async (item: ClipboardItem) => {
    try {
      setBusyItemId(item.id);
      await restoreClipboardHistoryItem({ id: item.id });
      setRestoredId(item.id);
      window.setTimeout(() => setRestoredId(null), 1400);
    } catch {
      setLoadState("failed");
    } finally {
      setBusyItemId(null);
    }
  };

  const deleteItem = async (item: ClipboardItem) => {
    try {
      setBusyItemId(item.id);
      setSnapshot(await deleteClipboardHistoryItem({ id: item.id }));
    } catch {
      setLoadState("failed");
    } finally {
      setBusyItemId(null);
    }
  };

  const clearAll = async () => {
    try {
      await clearClipboardHistory();
      await refresh();
    } catch {
      setLoadState("failed");
    }
  };

  const items = snapshot?.items ?? [];

  return (
    <section className="history-window" aria-label="剪贴板历史">
      <header className="history-header">
        <div>
          <h1>剪贴板历史</h1>
          <p>最近 {snapshot?.total ?? 0} 条记录</p>
        </div>
        <div className="history-toolbar">
          <button type="button" className="icon-button" onClick={() => void refresh()} aria-label="刷新">
            <RotateCcw size={16} aria-hidden="true" />
          </button>
          <button type="button" className="icon-button" onClick={closeWindow} aria-label="关闭历史窗口">
            <X size={18} aria-hidden="true" />
          </button>
        </div>
      </header>

      <main className="history-content">
        {loadState === "failed" ? <p className="history-status">历史记录操作失败，请稍后重试。</p> : null}
        {loadState === "loading" ? <p className="history-status">正在读取历史记录...</p> : null}
        {loadState === "ready" && items.length === 0 ? <p className="history-status">暂无剪贴板历史</p> : null}

        {items.length > 0 ? (
          <ul className="history-list">
            {items.map((item) => (
              <li key={item.id} className="history-item">
                <button
                  type="button"
                  className="history-item-main"
                  aria-label={`恢复 ${item.type === "text" ? "文本" : "图片"} ${item.preview}`}
                  onClick={() => void restoreItem(item)}
                  disabled={busyItemId === item.id}
                >
                  <span className="history-item-kind">{item.type === "text" ? "文本" : "图片"}</span>
                  {item.type === "image" ? (
                    <span className="history-image-preview">
                      {item.thumbnailPath ?? item.imagePath ? (
                        <img src={item.thumbnailPath ?? item.imagePath} alt={item.preview} />
                      ) : (
                        <ImageIcon size={22} aria-hidden="true" />
                      )}
                    </span>
                  ) : null}
                  <span className="history-item-preview">{item.preview}</span>
                  <span className="history-item-time">{formatTime(item.createdAt)}</span>
                </button>
                <div className="history-item-actions">
                  {restoredId === item.id ? (
                    <span className="history-restored" aria-label="已恢复">
                      <Check size={15} aria-hidden="true" />
                      <span>已恢复</span>
                    </span>
                  ) : null}
                  <button
                    type="button"
                    className="icon-button"
                    onClick={() => void deleteItem(item)}
                    disabled={busyItemId === item.id}
                    aria-label={`删除 ${item.preview}`}
                  >
                    <Trash2 size={15} aria-hidden="true" />
                  </button>
                </div>
              </li>
            ))}
          </ul>
        ) : null}
      </main>

      <footer className="history-footer">
        <span>容量 {snapshot?.settings.capacity ?? 20} 条</span>
        <button type="button" onClick={() => void clearAll()} disabled={items.length === 0}>
          清空历史
        </button>
      </footer>
    </section>
  );
}

function formatTime(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return "";
  return date.toLocaleString("zh-CN", {
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit"
  });
}
