# PRD01 桌宠入口与桌面交互 - Application Design

## 运行时职责
- Create and manage pet, history and settings windows.
- Persist pet settings locally.
- Expose Tauri commands consumed by the pet UI.
- Emit metadata-only pet events.

## 模块变更
- `packages/shared-contracts`: pet models, app event names and command DTOs.
- `apps/desktop`: Vite React app, pet feature and window entry.
- `src-tauri`: Tauri app bootstrap, commands, settings persistence, event helpers and window helpers.

## 数据流
1. App starts and creates pet window.
2. Pet UI calls `get_pet_settings`.
3. User drags the pet; UI moves the current window and calls `save_pet_position` on release.
4. User double-clicks; UI calls `open_history_panel`; Rust emits `pet.double_clicked` and opens/focuses history window.
5. User right-clicks; UI opens context menu and calls commands for selected actions.

## 存储 / Schema
- No SQLite migration for PRD01 MVP.
- JSON file: `pet-settings.json` in app data directory.
- Fields: `position`, `idleAnimationEnabled`, `autoMoveEnabled`, `alwaysOnTop`.

## 错误处理
- Invalid position input returns a command error.
- Persistence failures return a command error and keep current UI visible.
- Window creation/focus failure returns command error.

## 隐私与安全
- No clipboard content access.
- Pet events carry metadata only.
- No network calls.
- No model/API integration.

## CDD 契约

### 输入契约
- `PetPosition`: finite `x` and `y` values.
- `UpdatePetBehaviorSettingsRequest`: optional boolean flags.
- `SetRecordingPausedRequest`: `{ paused: boolean }`.

### 输出契约
- Commands return `PetSettings` or `null`.
- Errors return Tauri command error strings without sensitive content.

### 事件契约
- `pet.drag_started`, `pet.drag_ended`, `pet.double_clicked`, `pet.idle_started`, `pet.idle_moved`.
- Payload contains IDs, timestamps or position metadata only.

### 权限契约
- UI can request pet settings and window actions.
- UI cannot request clipboard/history content through pet commands.

### 错误契约
- `invalid_position`
- `settings_read_failed`
- `settings_write_failed`
- `window_operation_failed`

### 持久化契约
- Missing settings file returns defaults.
- Corrupt settings file falls back to defaults and overwrites on next successful save.
- Position is clamped by native layer before use when possible.

## 测试策略
- Shared contract tests validate event names and pet setting defaults.
- Rust unit tests validate default settings, save/read and invalid position rejection.
- Frontend tests validate pet component interactions.
- Build checks validate Tauri/Vite integration.
