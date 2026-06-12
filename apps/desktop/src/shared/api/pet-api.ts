import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { MovePetWindowByRequest, PetPosition, PetSettings, UpdatePetBehaviorSettingsRequest } from "@cute-clipboard/shared-contracts";
import { PET_COMMANDS } from "@cute-clipboard/shared-contracts";

export async function getPetSettings(): Promise<PetSettings> {
  return invoke<PetSettings>(PET_COMMANDS.getPetSettings);
}

export async function savePetPosition(position: PetPosition): Promise<PetSettings> {
  return invoke<PetSettings>(PET_COMMANDS.savePetPosition, { position });
}

export async function movePetWindowBy(request: MovePetWindowByRequest): Promise<PetPosition> {
  return invoke<PetPosition>(PET_COMMANDS.movePetWindowBy, { request });
}

export async function updatePetBehaviorSettings(input: UpdatePetBehaviorSettingsRequest): Promise<PetSettings> {
  return invoke<PetSettings>(PET_COMMANDS.updatePetBehaviorSettings, { input });
}

export async function openHistoryPanel(): Promise<void> {
  await invoke(PET_COMMANDS.openHistoryPanel);
}

export async function closeHistoryPanel(): Promise<void> {
  await invoke(PET_COMMANDS.closeHistoryPanel);
}

export async function openSettingsWindow(): Promise<void> {
  await invoke(PET_COMMANDS.openSettingsWindow);
}

export async function setRecordingPaused(paused: boolean): Promise<void> {
  await invoke(PET_COMMANDS.setRecordingPaused, { request: { paused } });
}

export async function quitApp(): Promise<void> {
  await invoke(PET_COMMANDS.quitApp);
}

export async function startWindowDrag(): Promise<void> {
  await getCurrentWindow().startDragging();
}

export async function currentWindowOuterPosition(): Promise<PetPosition> {
  const position = await getCurrentWindow().outerPosition();
  return { x: position.x, y: position.y };
}
