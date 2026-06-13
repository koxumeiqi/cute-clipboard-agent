export type HistoryCapacity = 10 | 20 | 50;

export interface AppSettings {
  historyCapacity: HistoryCapacity;
  recordText: boolean;
  recordImage: boolean;
  idleAnimationEnabled: boolean;
  autoMoveEnabled: boolean;
  launchAtStartup: boolean;
  persistenceEnabled: boolean;
  recordingPaused: boolean;
}

export const DEFAULT_APP_SETTINGS: AppSettings = {
  historyCapacity: 20,
  recordText: true,
  recordImage: true,
  idleAnimationEnabled: true,
  autoMoveEnabled: false,
  launchAtStartup: false,
  persistenceEnabled: true,
  recordingPaused: false
};

export function isHistoryCapacity(value: number): value is HistoryCapacity {
  return value === 10 || value === 20 || value === 50;
}
