# PRD01 桌宠入口与桌面交互 - UI Design

## 页面 / 窗口 / 入口
- `pet` window: 96x112 transparent window, no visible app chrome.
- `history` window: placeholder panel opened from double click.
- `settings` window: placeholder panel opened from context menu.

## 组件结构
- `PetWindow`: loads settings and wires API calls.
- `Pet`: visual pet, drag/double-click/right-click interactions.
- `PetContextMenu`: compact right-click command menu.

## 状态
- loading: pet visual is disabled until settings load.
- ready: pet accepts drag, double click, right click.
- empty: not applicable.
- error: pet remains visible, commands disabled where necessary.
- disabled / unauthorized: not applicable for MVP pet entry.

## 交互
- Left mouse down starts drag unless it becomes a double click.
- Mouse move during drag uses current Tauri window drag APIs.
- Mouse up saves last known position.
- Double click calls `open_history_panel`.
- Right click opens a local context menu near the pointer.
- Context menu actions call shared API wrappers.

## 视觉与可访问性规划
- Use CSS pet shape with stable dimensions to avoid layout shift.
- Keep controls icon-first where possible; context menu text is concise.
- Use `aria-label` on the pet button and menu actions.
- Avoid decorative cards inside cards; pet window is the primary surface.

## UI BDD 场景

### 场景: Pet ready state renders stable surface
Given settings load successfully
When the pet window renders
Then the pet interactive surface has stable dimensions
And it exposes an accessible label

### 场景: Double click calls history command
Given pet is ready
When the user double clicks the pet
Then `open_history_panel` is called once

### 场景: Right click opens context menu
Given pet is ready
When the user opens the context menu
Then menu actions for history, settings, pause and exit are visible

### 场景: Idle animation disabled
Given `idleAnimationEnabled` is false
When pet is ready
Then animated visual class is not applied

## E2E 验证计划

### 自动化范围
- Unit/component tests for render state, double click, context menu and animation flag.
- Rust unit tests for settings defaults and persisted position.
- Build/typecheck for Tauri/Vite wiring.

### 人工范围
- Real Windows transparent borderless window.
- Real drag movement and restart position restore.
- Real right-click native menu if later changed from HTML menu to Tauri menu.

### 环境要求
- Windows with WebView2 Runtime.
- Tauri dev/build toolchain.

### 需要捕获的证据
- Command output for automated checks.
- Manual checklist result for Windows desktop behavior.

## 人工验证清单
- 启动 `npm run tauri dev` 后，桌宠显示在屏幕可见区域且没有系统边框。
- 拖拽桌宠到新位置，释放后停留在该位置。
- 重启应用，桌宠恢复到上次位置或在可见区域内。
- 双击桌宠后历史窗口打开或聚焦。
- 右键桌宠后菜单可见，菜单项可以触发对应占位动作。
- 关闭 idle animation 后，桌宠不再播放 idle 动画。
