import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import { DEFAULT_PET_SETTINGS } from "@cute-clipboard/shared-contracts";
import { Pet } from "./pet";

afterEach(() => {
  cleanup();
});

function renderPet(settings = DEFAULT_PET_SETTINGS) {
  const onDragMove = vi.fn();
  const onDragEnd = vi.fn();
  const onOpenHistory = vi.fn();
  const onOpenSettings = vi.fn();
  const onPauseRecording = vi.fn();
  const onQuit = vi.fn();

  return {
    onDragMove,
    onDragEnd,
    onOpenHistory,
    onOpenSettings,
    onPauseRecording,
    onQuit,
    user: userEvent.setup(),
    ...render(
      <Pet
        settings={settings}
        onDragMove={onDragMove}
        onDragEnd={onDragEnd}
        onOpenHistory={onOpenHistory}
        onOpenSettings={onOpenSettings}
        onPauseRecording={onPauseRecording}
        onQuit={onQuit}
      />
    )
  };
}

describe("Pet", () => {
  const petName = "桌宠剪贴板助手";

  it("renders a stable accessible pet surface", () => {
    renderPet();
    expect(screen.getByRole("button", { name: petName })).toBeInTheDocument();
  });

  it("moves the window only after the pointer crosses the drag threshold", () => {
    const { onDragMove } = renderPet();
    const pet = screen.getByRole("button", { name: petName });

    fireEvent.mouseDown(pet, { button: 0, screenX: 20, screenY: 20 });
    fireEvent.mouseMove(pet, { screenX: 22, screenY: 22 });
    fireEvent.mouseMove(pet, { screenX: 36, screenY: 42 });
    fireEvent.mouseUp(pet, { screenX: 36, screenY: 42 });

    expect(onDragMove).toHaveBeenCalledTimes(1);
    expect(onDragMove).toHaveBeenCalledWith(16, 22);
  });

  it("does not treat a drag as a double click", () => {
    const { onOpenHistory } = renderPet();
    const pet = screen.getByRole("button", { name: petName });

    fireEvent.mouseDown(pet, { button: 0, screenX: 20, screenY: 20 });
    fireEvent.mouseMove(pet, { screenX: 40, screenY: 40 });
    fireEvent.mouseUp(pet, { screenX: 40, screenY: 40 });
    fireEvent.mouseDown(pet, { button: 0, screenX: 21, screenY: 21 });
    fireEvent.mouseUp(pet, { screenX: 21, screenY: 21 });

    expect(onOpenHistory).not.toHaveBeenCalled();
  });

  it("calls history action on double click", async () => {
    const { onOpenHistory } = renderPet();
    const pet = screen.getByRole("button", { name: petName });

    fireEvent.mouseDown(pet, { button: 0, screenX: 20, screenY: 20 });
    fireEvent.mouseUp(pet, { screenX: 20, screenY: 20 });
    fireEvent.mouseDown(pet, { button: 0, screenX: 22, screenY: 22 });
    fireEvent.mouseUp(pet, { screenX: 22, screenY: 22 });

    await waitFor(() => expect(onOpenHistory).toHaveBeenCalledTimes(1));
  });

  it("opens context menu with expected actions", async () => {
    const { user } = renderPet();

    await user.pointer({
      keys: "[MouseRight]",
      target: screen.getByRole("button", { name: petName })
    });

    expect(screen.getByRole("menu")).toBeInTheDocument();
    expect(screen.getAllByRole("menuitem")).toHaveLength(4);
  });

  it("does not apply idle animation class when disabled by settings", () => {
    renderPet({ ...DEFAULT_PET_SETTINGS, idleAnimationEnabled: false });
    expect(screen.getByRole("button", { name: petName })).not.toHaveClass("pet-surface--animated");
  });
});
