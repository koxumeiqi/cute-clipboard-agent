import type { KeyboardEvent, MouseEvent } from "react";
import { useEffect, useRef, useState } from "react";
import type { PetSettings, PetState } from "@cute-clipboard/shared-contracts";
import { PetContextMenu } from "./pet-context-menu";

export interface PetProps {
  settings: PetSettings;
  disabled?: boolean;
  onDragMove: (deltaX: number, deltaY: number) => Promise<void> | void;
  onDragEnd: () => Promise<void> | void;
  onOpenHistory: () => void;
  onOpenSettings: () => void;
  onPauseRecording: () => void;
  onQuit: () => void;
}

type DragSession = {
  startX: number;
  startY: number;
  lastX: number;
  lastY: number;
  moved: boolean;
};

const DRAG_THRESHOLD_PX = 4;
const DOUBLE_CLICK_MS = 420;
const DOUBLE_CLICK_DISTANCE_PX = 12;

export function Pet({
  settings,
  disabled = false,
  onDragMove,
  onDragEnd,
  onOpenHistory,
  onOpenSettings,
  onPauseRecording,
  onQuit
}: PetProps) {
  const [state, setState] = useState<PetState>("idle");
  const [menuPosition, setMenuPosition] = useState<{ x: number; y: number } | null>(null);
  const dragSessionRef = useRef<DragSession | null>(null);
  const dragCleanupRef = useRef<(() => void) | null>(null);
  const suppressClickUntilRef = useRef(0);
  const openingHistoryRef = useRef(false);
  const lastClickRef = useRef<{ time: number; x: number; y: number } | null>(null);
  const animated = settings.idleAnimationEnabled && state === "idle";

  const openHistory = () => {
    if (disabled || openingHistoryRef.current) return;
    openingHistoryRef.current = true;
    setMenuPosition(null);
    setState("opening_panel");
    window.setTimeout(() => onOpenHistory(), 0);
    window.setTimeout(() => {
      openingHistoryRef.current = false;
      setState("idle");
    }, 120);
  };

  useEffect(() => {
    const handleNativeDoubleClick = (event: globalThis.MouseEvent) => {
      if (isMenuTarget(event.target)) return;
      if (!(event.target instanceof Element) || !event.target.closest(".pet-shell")) return;
      event.preventDefault();
      openHistory();
    };

    document.addEventListener("dblclick", handleNativeDoubleClick);
    return () => document.removeEventListener("dblclick", handleNativeDoubleClick);
  }, [disabled, onOpenHistory]);

  const isMenuTarget = (target: EventTarget | null) =>
    target instanceof Element && target.closest(".pet-menu");

  const clearDocumentDragListeners = () => {
    dragCleanupRef.current?.();
    dragCleanupRef.current = null;
  };

  const updateDrag = (screenX: number, screenY: number) => {
    const session = dragSessionRef.current;
    if (!session || disabled) return;

    const totalDeltaX = screenX - session.startX;
    const totalDeltaY = screenY - session.startY;
    if (Math.hypot(totalDeltaX, totalDeltaY) < DRAG_THRESHOLD_PX) return;

    const deltaX = Math.round(screenX - session.lastX);
    const deltaY = Math.round(screenY - session.lastY);
    if (deltaX === 0 && deltaY === 0) return;

    session.lastX = screenX;
    session.lastY = screenY;
    session.moved = true;
    lastClickRef.current = null;
    suppressClickUntilRef.current = window.performance.now() + 600;
    setState("dragging");
    void Promise.resolve(onDragMove(deltaX, deltaY)).catch(() => setState("idle"));
  };

  const finishDrag = (screenX: number, screenY: number) => {
    updateDrag(screenX, screenY);

    const session = dragSessionRef.current;
    dragSessionRef.current = null;
    clearDocumentDragListeners();
    if (!session) return false;

    const moved =
      session.moved ||
      Math.hypot(screenX - session.startX, screenY - session.startY) >= DRAG_THRESHOLD_PX;
    if (moved) {
      suppressClickUntilRef.current = window.performance.now() + 450;
      void Promise.resolve(onDragEnd()).finally(() => setState("idle"));
      return true;
    }

    setState("idle");
    return false;
  };

  const handleMouseDown = (event: MouseEvent<HTMLDivElement>) => {
    if (isMenuTarget(event.target)) return;
    if (disabled || event.button !== 0) return;
    event.currentTarget.focus();
    setMenuPosition(null);
    clearDocumentDragListeners();
    dragSessionRef.current = {
      startX: event.screenX,
      startY: event.screenY,
      lastX: event.screenX,
      lastY: event.screenY,
      moved: false
    };

    const handleDocumentMouseMove = (nativeEvent: globalThis.MouseEvent) => {
      if (nativeEvent.button !== 0 && nativeEvent.buttons !== 1) return;
      nativeEvent.preventDefault();
      updateDrag(nativeEvent.screenX, nativeEvent.screenY);
    };
    const handleDocumentMouseUp = (nativeEvent: globalThis.MouseEvent) => {
      nativeEvent.preventDefault();
      finishDrag(nativeEvent.screenX, nativeEvent.screenY);
    };
    document.addEventListener("mousemove", handleDocumentMouseMove);
    document.addEventListener("mouseup", handleDocumentMouseUp);
    dragCleanupRef.current = () => {
      document.removeEventListener("mousemove", handleDocumentMouseMove);
      document.removeEventListener("mouseup", handleDocumentMouseUp);
    };
  };

  const handleMouseMove = (event: MouseEvent<HTMLDivElement>) => {
    updateDrag(event.screenX, event.screenY);
  };

  const handleMouseUp = (event: MouseEvent<HTMLDivElement>) => {
    const moved = finishDrag(event.screenX, event.screenY);
    if (moved) {
      return;
    }

    if (window.performance.now() < suppressClickUntilRef.current) {
      return;
    }

    const now = window.performance.now();
    const previousClick = lastClickRef.current;
    const clickedNearPrevious =
      previousClick &&
      now - previousClick.time <= DOUBLE_CLICK_MS &&
      Math.hypot(event.screenX - previousClick.x, event.screenY - previousClick.y) <= DOUBLE_CLICK_DISTANCE_PX;

    if (clickedNearPrevious) {
      lastClickRef.current = null;
      openHistory();
      return;
    }

    lastClickRef.current = {
      time: now,
      x: event.screenX,
      y: event.screenY
    };
  };

  const handleMouseLeave = () => {
    if (!dragSessionRef.current) {
      setState("idle");
    }
  };

  const handleContextMenu = (event: MouseEvent<HTMLDivElement>) => {
    event.preventDefault();
    if (disabled) return;
    setMenuPosition({ x: Math.min(event.clientX, 18), y: Math.min(event.clientY, 54) });
  };

  const handleDoubleClick = (event: MouseEvent<HTMLDivElement>) => {
    if (isMenuTarget(event.target)) return;
    event.preventDefault();
    if (window.performance.now() < suppressClickUntilRef.current) return;
    lastClickRef.current = null;
    openHistory();
  };

  const runMenuAction = (action: () => void) => {
    setMenuPosition(null);
    action();
  };

  const handleKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    if (disabled) return;
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      openHistory();
    }
  };

  return (
    <div
      role="button"
      tabIndex={disabled ? -1 : 0}
      className="pet-shell"
      aria-label="桌宠剪贴板助手"
      aria-disabled={disabled}
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseLeave}
      onKeyDown={handleKeyDown}
      onContextMenu={handleContextMenu}
      onDoubleClick={handleDoubleClick}
    >
      <div className={`pet-surface${animated ? " pet-surface--animated" : ""}`}>
        <span className="pet-ear pet-ear--left" />
        <span className="pet-ear pet-ear--right" />
        <span className="pet-face">
          <span className="pet-eye" />
          <span className="pet-eye" />
          <span className="pet-mouth" />
        </span>
      </div>
      {menuPosition ? (
        <PetContextMenu
          x={menuPosition.x}
          y={menuPosition.y}
          onOpenHistory={() => runMenuAction(onOpenHistory)}
          onOpenSettings={() => runMenuAction(onOpenSettings)}
          onPauseRecording={() => runMenuAction(onPauseRecording)}
          onQuit={() => runMenuAction(onQuit)}
        />
      ) : null}
    </div>
  );
}
