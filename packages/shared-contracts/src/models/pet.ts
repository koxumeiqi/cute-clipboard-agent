export interface PetPosition {
  x: number;
  y: number;
}

export interface PetSettings {
  position: PetPosition;
  idleAnimationEnabled: boolean;
  autoMoveEnabled: boolean;
  alwaysOnTop: boolean;
}

export type PetState = "idle" | "dragging" | "opening_panel";

export const DEFAULT_PET_SETTINGS: PetSettings = {
  position: { x: 80, y: 120 },
  idleAnimationEnabled: true,
  autoMoveEnabled: false,
  alwaysOnTop: true
};

export function isFinitePetPosition(position: PetPosition): boolean {
  return Number.isFinite(position.x) && Number.isFinite(position.y);
}
