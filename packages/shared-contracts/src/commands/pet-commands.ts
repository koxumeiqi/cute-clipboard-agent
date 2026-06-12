import type { PetPosition } from "../models/pet";

export const PET_COMMANDS = {
  getPetSettings: "get_pet_settings",
  savePetPosition: "save_pet_position",
  movePetWindowBy: "move_pet_window_by",
  updatePetBehaviorSettings: "update_pet_behavior_settings",
  openHistoryPanel: "open_history_panel",
  closeHistoryPanel: "close_history_panel",
  openSettingsWindow: "open_settings_window",
  setRecordingPaused: "set_recording_paused",
  showPetContextMenu: "show_pet_context_menu",
  quitApp: "quit_app"
} as const;

export interface UpdatePetBehaviorSettingsRequest {
  idleAnimationEnabled?: boolean;
  autoMoveEnabled?: boolean;
  alwaysOnTop?: boolean;
}

export interface SetRecordingPausedRequest {
  paused: boolean;
}

export interface SavePetPositionRequest {
  position: PetPosition;
}

export interface MovePetWindowByRequest {
  deltaX: number;
  deltaY: number;
}
