import { describe, expect, it } from "vitest";
import { DEFAULT_PET_SETTINGS, isFinitePetPosition, PET_COMMANDS, PET_EVENT_NAMES } from "./index";

describe("pet shared contracts", () => {
  it("keeps MVP pet defaults local and stable", () => {
    expect(DEFAULT_PET_SETTINGS).toEqual({
      position: { x: 80, y: 120 },
      idleAnimationEnabled: true,
      autoMoveEnabled: false,
      alwaysOnTop: true
    });
  });

  it("defines metadata-only pet event names", () => {
    expect(PET_EVENT_NAMES.doubleClicked).toBe("pet.double_clicked");
    expect(PET_EVENT_NAMES.dragEnded).toBe("pet.drag_ended");
  });

  it("defines Tauri command names used by the pet UI", () => {
    expect(PET_COMMANDS.openHistoryPanel).toBe("open_history_panel");
    expect(PET_COMMANDS.savePetPosition).toBe("save_pet_position");
  });

  it("rejects non-finite positions", () => {
    expect(isFinitePetPosition({ x: 1, y: 2 })).toBe(true);
    expect(isFinitePetPosition({ x: Number.NaN, y: 2 })).toBe(false);
  });
});
