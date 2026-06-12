# 剪贴板历史面板 - 应用设计

## 运行时职责
- 前端 history 面板负责展示状态和触发用户操作。
- `shared/api/clipboard-api.ts` 负责封装 Tauri command。
- `src-tauri/src/commands` 负责参数 DTO、错误映射与调用领域模块。
- `history` 模块负责查询、删除、清空和容量裁剪。
- `clipboard` 模块负责写回系统剪贴板与写回抑制。

## 模块变更
- `features/history-panel`：修正文案、图片预览、恢复/删除/清空反馈和测试。
- `shared/api`：沿用已有 API 封装。
- `commands`：补齐图片恢复写回逻辑。
- `clipboard`：提供从历史图片路径写回系统剪贴板的辅助函数。

## 数据流
1. 用户打开窗口。
2. UI 调用 `list_clipboard_history`。
3. Rust 返回 `ClipboardHistorySnapshot`。
4. UI 渲染列表。
5. 用户点击历史项。
6. UI 调用 `restore_clipboard_history_item`。
7. Rust 获取历史项，按类型写回剪贴板，并将该项 hash 放入写回抑制队列。
8. UI 展示恢复成功反馈。

## 存储 / Schema
- 不新增 SQLite 表。
- 删除与清空沿用现有 history store，并继续清理图片文件。

## 错误处理
- 读取失败：UI 显示错误状态并保留刷新入口。
- 恢复失败：UI 显示错误提示，不关闭窗口。
- 删除失败：UI 显示错误提示，保留当前列表。
- 清空失败：UI 显示错误提示，保留当前列表。

## 隐私与安全
- 完整文本只在用户打开面板和恢复时经本地 Tauri command 流转。
- 事件 payload 不携带完整文本或原图数据。
- 不新增网络调用。
- 不输出完整剪贴板内容到日志。

## CDD 契约

### 输入契约
- `ClipboardHistoryItemRequest`：`{ id: string }`。
- `UpdateClipboardHistorySettingsRequest`：本 PRD 不调整。

### 输出契约
- `ClipboardHistorySnapshot`：`items` 最新在前，`total` 等于当前条数。
- `clear_clipboard_history`：返回删除数量。
- `restore_clipboard_history_item`：成功返回空结果，失败返回错误字符串。

### 事件契约
- 删除单条：`clipboard.deleted`，只携带记录 id。
- 清空：`history.cleared`，只携带删除数量。
- 恢复：不新增事件。

### 权限契约
- 用户主动打开的本地窗口可读取完整历史项。
- Agent Bridge 默认不可读取完整内容。

### 错误契约
- `history_item_not_found`
- `clipboard_write_failed`
- `unsupported_clipboard_restore`
- `clipboard_recorder_lock_failed`

### 持久化契约
- `persistEnabled=true` 时历史项持久化。
- `persistEnabled=false` 时新历史项不持久化，但当前内存快照仍可展示。

## 测试策略
- 前端组件测试：加载列表、空态、图片预览、恢复、删除、清空。
- Rust 单元测试：图片路径解析/恢复辅助逻辑能拒绝缺失路径与无效图片。
- 集成/人工验证：Windows 剪贴板真实写回文本和图片。
