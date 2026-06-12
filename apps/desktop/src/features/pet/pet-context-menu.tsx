import { History, Pause, Settings, X } from "lucide-react";

export interface PetContextMenuProps {
  x: number;
  y: number;
  onOpenHistory: () => void;
  onOpenSettings: () => void;
  onPauseRecording: () => void;
  onQuit: () => void;
}

export function PetContextMenu({
  x,
  y,
  onOpenHistory,
  onOpenSettings,
  onPauseRecording,
  onQuit
}: PetContextMenuProps) {
  return (
    <div className="pet-menu" style={{ left: x, top: y }} role="menu" aria-label="桌宠快捷菜单">
      <button type="button" role="menuitem" onClick={onOpenHistory}>
        <History size={16} aria-hidden="true" />
        <span>打开历史</span>
      </button>
      <button type="button" role="menuitem" onClick={onOpenSettings}>
        <Settings size={16} aria-hidden="true" />
        <span>设置</span>
      </button>
      <button type="button" role="menuitem" onClick={onPauseRecording}>
        <Pause size={16} aria-hidden="true" />
        <span>暂停记录</span>
      </button>
      <button type="button" role="menuitem" onClick={onQuit}>
        <X size={16} aria-hidden="true" />
        <span>退出</span>
      </button>
    </div>
  );
}
