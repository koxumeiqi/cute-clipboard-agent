import { cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { AppSettings } from "@cute-clipboard/shared-contracts";
import { SettingsWindow } from "./settings-window";

const settingsApi = vi.hoisted(() => ({
  getAppSettings: vi.fn(),
  updateAppSettings: vi.fn()
}));

const windowApi = vi.hoisted(() => ({
  close: vi.fn()
}));

const labels = {
  settings: "\u8bbe\u7f6e",
  capacity20: "20 \u6761",
  capacity10: "10 \u6761",
  recordText: "\u8bb0\u5f55\u6587\u672c\u548c emoji",
  recordImage: "\u8bb0\u5f55\u56fe\u7247",
  pauseRecording: "\u6682\u505c\u8bb0\u5f55",
  saved: "\u5df2\u4fdd\u5b58",
  saveFailed: "\u8bbe\u7f6e\u4fdd\u5b58\u5931\u8d25\uff0c\u8bf7\u7a0d\u540e\u91cd\u8bd5\u3002"
} as const;

vi.mock("../../shared/api/settings-api", () => settingsApi);
vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => windowApi
}));

afterEach(() => {
  cleanup();
  vi.clearAllMocks();
});

describe("SettingsWindow", () => {
  it("loads and renders current settings", async () => {
    settingsApi.getAppSettings.mockResolvedValue(appSettings());

    render(<SettingsWindow />);

    expect(await screen.findByRole("heading", { name: labels.settings })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: labels.capacity20 })).toHaveAttribute("aria-pressed", "true");
    expect(screen.getByRole("switch", { name: labels.recordText })).toBeChecked();
    expect(screen.getByRole("switch", { name: labels.recordImage })).toBeChecked();
    expect(screen.getByRole("switch", { name: labels.pauseRecording })).not.toBeChecked();
  });

  it("saves capacity changes immediately", async () => {
    settingsApi.getAppSettings.mockResolvedValue(appSettings());
    settingsApi.updateAppSettings.mockResolvedValue({ ...appSettings(), historyCapacity: 10 });
    const user = userEvent.setup();

    render(<SettingsWindow />);
    await user.click(await screen.findByRole("button", { name: labels.capacity10 }));

    expect(settingsApi.updateAppSettings).toHaveBeenCalledWith({ historyCapacity: 10 });
    await waitFor(() => expect(screen.getByRole("button", { name: labels.capacity10 })).toHaveAttribute("aria-pressed", "true"));
    expect(screen.getByText(labels.saved)).toBeInTheDocument();
  });

  it("saves recording pause changes immediately", async () => {
    settingsApi.getAppSettings.mockResolvedValue(appSettings());
    settingsApi.updateAppSettings.mockResolvedValue({ ...appSettings(), recordingPaused: true });
    const user = userEvent.setup();

    render(<SettingsWindow />);
    await user.click(await screen.findByRole("switch", { name: labels.pauseRecording }));

    expect(settingsApi.updateAppSettings).toHaveBeenCalledWith({ recordingPaused: true });
    await waitFor(() => expect(screen.getByRole("switch", { name: labels.pauseRecording })).toBeChecked());
  });

  it("restores previous settings when save fails", async () => {
    settingsApi.getAppSettings.mockResolvedValue(appSettings());
    settingsApi.updateAppSettings.mockRejectedValue(new Error("failed"));
    const user = userEvent.setup();

    render(<SettingsWindow />);
    await user.click(await screen.findByRole("switch", { name: labels.recordText }));

    await waitFor(() => expect(screen.getByText(labels.saveFailed)).toBeInTheDocument());
    expect(screen.getByRole("switch", { name: labels.recordText })).toBeChecked();
  });
});

function appSettings(): AppSettings {
  return {
    historyCapacity: 20,
    recordText: true,
    recordImage: true,
    idleAnimationEnabled: true,
    autoMoveEnabled: false,
    launchAtStartup: false,
    persistenceEnabled: true,
    recordingPaused: false
  };
}
