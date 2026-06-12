# PRD01 桌宠入口与桌面交互 - Base Design

## 来源
- PRD: `docs/prd/01-桌宠入口与桌面交互.md`
- Branch: `codex/prd-01-pet-entry-desktop-interaction`
- Related modules: `apps/desktop/src/features/pet`, `apps/desktop/src/windows/pet`, `src-tauri/src/windows`, `src-tauri/src/settings`, `src-tauri/src/events`, `packages/shared-contracts`

## 目标
- 提供一个透明、无边框、常驻置顶的桌宠窗口。
- 支持拖拽移动、释放后保存位置，并在启动时恢复。
- 支持双击桌宠打开或聚焦历史面板。
- 支持右键快捷菜单，提供打开历史、设置、暂停记录、退出等入口占位。
- 支持 idle animation 开关和 auto move 开关，MVP 默认 auto move 关闭。

## 非目标
- 不实现 Live2D、多角色、复杂动画资产。
- 不实现剪贴板监听、历史存储、历史恢复业务。
- 不接入真实 AI、OpenAI API、本地模型或 MCP。
- 不实现边缘吸附、避让窗口、多屏复杂策略。

## 共享领域模型
- `PetPosition`: `{ x: number; y: number }`
- `PetSettings`: `{ position: PetPosition; idleAnimationEnabled: boolean; autoMoveEnabled: boolean; alwaysOnTop: boolean }`
- `PetState`: `idle | dragging | opening_panel`

## 公共契约

### Commands / APIs
- `get_pet_settings(): PetSettings`
- `save_pet_position(position: PetPosition): PetSettings`
- `update_pet_behavior_settings(input): PetSettings`
- `open_history_panel(): void`
- `open_settings_window(): void`
- `set_recording_paused(paused: boolean): void`
- `show_pet_context_menu(): void`

### Events
- `pet.drag_started`: metadata only, `{ at: string }`
- `pet.drag_ended`: metadata only, `{ position: PetPosition; at: string }`
- `pet.double_clicked`: metadata only, `{ at: string }`
- `pet.idle_started`: metadata only, `{ at: string }`
- `pet.idle_moved`: metadata only, reserved for auto move, `{ position: PetPosition; at: string }`

### Permissions
- Pet UI has no clipboard content permission.
- Events must not include full clipboard text or image bytes.
- Agent Bridge is not invoked by this PRD.

### Persistence
- MVP stores pet settings in a local JSON file under app data via Rust settings service.
- Invalid or offscreen positions fall back to default visible position.

## UI-App 集成契约

### UI 调用
- Pet UI uses shared API wrappers, not direct scattered `invoke` calls.
- Drag movement uses Tauri window APIs for live movement and calls command only after release.

### 应用 / 原生响应
- Native layer creates the `pet` window as transparent, undecorated, always-on-top and non-resizable.
- `open_history_panel` creates/shows/focuses a `history` window placeholder.

### 事件订阅
- Pet UI does not subscribe to clipboard/history content.
- Future modules may subscribe to `pet.double_clicked` to coordinate panel behavior.

### 状态同步
- Initial pet settings load at window mount.
- UI updates idle animation and auto move flags from settings response.

### 错误映射
- Settings read/write errors map to user-visible lightweight disabled/error state.
- Window operation errors are logged as command errors, without exposing clipboard content.

### 权限边界
- Pet can request window actions and settings updates only.
- Pet cannot read clipboard or history entries.

## 集成边界
- `features/pet` owns visual and interaction state.
- `windows/pet` owns the pet window entry composition.
- Rust `windows` module owns Tauri window creation/show/focus.
- Rust `settings` module owns persistence.
- Rust `events` module owns metadata-only app events.

## BDD 场景

### 场景: 启动后显示桌宠
Given 用户启动应用
When pet window is created
Then 桌宠在可见区域显示
And 桌宠窗口无系统边框

### 场景: 拖拽并保存位置
Given 桌宠显示在默认位置
When 用户拖拽桌宠并释放
Then 桌宠停留在新位置
And 新位置被保存用于下次启动恢复

### 场景: 双击打开历史面板
Given 桌宠处于 idle 状态
When 用户双击桌宠
Then 系统发布 `pet.double_clicked`
And 历史面板窗口被打开或聚焦

### 场景: 右键显示快捷菜单
Given 桌宠处于 idle 状态
When 用户右键桌宠
Then 显示桌宠快捷菜单

### 场景: 关闭 idle animation
Given idle animation 已关闭
When 桌宠进入 idle 状态
Then 桌宠不播放 idle 动画

## 成功标准
| 标准 | 来源 | 自动化证据 | 人工证据 |
|---|---|---|---|
| 启动后桌宠可见且无系统边框 | AC-PET-01 | Tauri config / command test | Windows 手工启动观察 |
| 拖拽释放后保存新位置 | AC-PET-02/03 | Rust settings tests, UI tests | 重启应用观察位置 |
| 双击触发历史面板打开 | AC-PET-04 | UI click test / command test | 双击桌宠观察历史面板 |
| 右键显示快捷菜单 | AC-PET-05 | UI state test | 右键桌宠观察菜单 |
| 关闭 idle animation 后不播放 | AC-PET-06 | UI class/state test | 关闭后观察 |

## 风险
- Windows 多屏可见区域策略在 MVP 中只做基础夹取，复杂多屏需后续增强。
- 当前仓库是骨架工程，需要先补齐最小 Tauri/Vite 工程。

## 未决问题
- 桌宠视觉资产尚未最终确认，本实现采用 CSS 绘制的轻量桌宠形象作为 MVP 占位。
