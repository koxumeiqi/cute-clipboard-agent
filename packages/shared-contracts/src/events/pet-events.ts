import type { PetPosition } from "../models/pet";

export const PET_EVENT_NAMES = {
  dragStarted: "pet.drag_started",
  dragEnded: "pet.drag_ended",
  doubleClicked: "pet.double_clicked",
  idleStarted: "pet.idle_started",
  idleMoved: "pet.idle_moved"
} as const;

export type PetEventName = (typeof PET_EVENT_NAMES)[keyof typeof PET_EVENT_NAMES];

export interface PetEventBasePayload {
  at: string;
}

export interface PetDragEndedPayload extends PetEventBasePayload {
  position: PetPosition;
}

export interface PetIdleMovedPayload extends PetEventBasePayload {
  position: PetPosition;
}
