# 设置与用户偏好 - 应用设计

## 运行时职责
- `settings` 模块定义聚合 `AppSettings`、启动偏好和更新请求。
- `commands` 模块暴露读取和保存设置命令。
- `events` 模块发送设置更新、暂停、恢复事件。
- `windows` 模块打开独立设置窗口。

## 模块变更
- `packages/shared-contracts` 新增 settings 模型、commands 和 events。
- `src-tauri/src/settings` 保留 PetSettings，并新增 `AppSettingsStore` 聚合逻辑。
- `apps/desktop/src/shared/api` 新增 `settings-api.ts`。
- `apps/desktop/src/features/settings` 新增设置窗口组件和测试。

## 数据流
1. 设置页调用 `get_app_settings`。
2. Rust 聚合 pet、clipboard、history、startup 偏好。
3. 用户更新字段后调用 `update_app_settings`。
4. Rust 分发到对应 store。
5. 成功后发送 `settings.updated`，并返回最新聚合设置。

## 存储 / Schema
- history settings 继续使用 SQLite `history_settings`。
- clipboard recording settings 继续使用 `clipboard-recording-settings.json`。
- pet settings 继续使用 `pet-settings.json`。
- startup preference 使用 `app-preferences.json`，当前仅包含 `launchAtStartup`。

## 错误处理
- 任一 store 更新失败则命令返回错误。
- 容量非法返回 `invalid_history_capacity`。
- JSON 读写失败返回 `settings_read_failed` 或 `settings_write_failed`。

## 隐私与安全
- 设置持久化仅在本地应用数据目录。
- 设置事件不携带剪贴板完整内容。
- 暂停记录和记录类型设置由剪贴板监听模块在入栈前检查。

## CDD 契约
### 输入契约
- `UpdateAppSettingsRequest`：
  - `historyCapacity?: 10 | 20 | 50`
  - `recordText?: boolean`
  - `recordImage?: boolean`
  - `idleAnimationEnabled?: boolean`
  - `autoMoveEnabled?: boolean`
  - `launchAtStartup?: boolean`
  - `persistenceEnabled?: boolean`
  - `recordingPaused?: boolean`

### 输出契约
- `AppSettings` 包含上述所有字段，且字段名使用 camelCase。

### 事件契约
- `settings.updated` payload：`{ at, settings }`
- `settings.clipboard_recording_paused` payload：`{ at }`
- `settings.clipboard_recording_resumed` payload：`{ at }`

### 权限契约
- 无网络权限。
- 无 Agent 完整内容访问权限变更。

### 错误契约
- `invalid_history_capacity`
- `settings_read_failed`
- `settings_write_failed`
- `history_storage_failed`
- `window_operation_failed`

### 持久化契约
- 保存成功后重启应用仍可读取相同设置。

## 测试策略
- Rust：测试 `AppPreferences` 默认值、持久化、`AppSettings` 聚合映射。
- 前端：测试设置页加载、切换保存、失败恢复。
- 既有 clipboard/history 测试覆盖暂停、记录类型和容量裁剪。
