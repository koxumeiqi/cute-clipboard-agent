# 剪贴板历史栈与本地存储 - 应用设计

## 运行时职责

历史服务负责接收归一化后的 `ClipboardItem`，维护内存倒序列表，根据设置持久化到 SQLite，并在删除、清空和裁剪时清理图片文件。

## 模块变更

- `src-tauri/src/history`：新增历史服务、设置、错误、容量校验、入栈裁剪、查询、删除、清空。
- `src-tauri/src/storage`：新增 SQLite 连接路径、schema 初始化和 repository。
- `src-tauri/src/image`：新增图片文件删除工具。
- `src-tauri/src/events`：新增 `clipboard.deleted` 和 `history.cleared`。
- `src-tauri/src/commands`：新增历史查询、删除、清空和设置 commands。
- `packages/shared-contracts`：新增历史 commands、事件和 DTO。

## 数据流

剪贴板监听读取原始内容 -> 剪贴板模块归一化为 `ClipboardItem` -> 历史服务 `push` 入栈和持久化 -> 裁剪旧记录和清理图片文件 -> 发送 `clipboard.created`。

## 存储 / Schema

SQLite 文件位于应用数据目录 `cute-clipboard-agent.sqlite3`。初始 schema：

- `clipboard_history_items`：`id`、`item_type`、`preview`、`text`、`image_path`、`thumbnail_path`、`hash`、`created_at`、`updated_at`。
- `history_settings`：`id`、`capacity`、`persist_enabled`、`updated_at`。

## 错误处理

- 容量不在 10、20、50 时返回 `invalid_history_capacity`。
- 查询不存在 ID 返回 `history_item_not_found`。
- SQLite 或文件系统失败返回 `history_storage_failed` 或 `image_cleanup_failed`。
- 文件不存在视为清理成功，避免清空操作被已缺失文件阻断。

## 隐私与安全

所有数据默认保存在本地应用数据目录。事件不广播完整文本。清空历史同时清理数据库记录和图片文件，持久化关闭后新条目不写入 SQLite。

## CDD 契约

### 输入契约

- `UpdateClipboardHistorySettingsRequest.capacity` 可选，只允许 10、20、50。
- `UpdateClipboardHistorySettingsRequest.persistEnabled` 可选。
- `GetClipboardHistoryItemRequest.id` 和 `DeleteClipboardHistoryItemRequest.id` 必填且非空。

### 输出契约

- 列表按 `createdAt` 倒序返回。
- 删除和清空返回剩余列表或删除数量，不返回被删除的完整文本。

### 事件契约

- `clipboard.created` 在入栈成功后发出。
- `clipboard.deleted` payload：`{ at, id }`。
- `history.cleared` payload：`{ at, deletedCount }`。

### 权限契约

完整历史内容只能通过显式 command 获取；Agent Bridge MVP 无读取完整内容权限。

### 错误契约

错误字符串稳定，不包含完整文本或图片字节。

### 持久化契约

持久化开启：启动时从 SQLite 加载最近 `capacity` 条。  
持久化关闭：新条目只进入内存；重启不恢复关闭期间产生的历史。

## 测试策略

- Rust 单元测试覆盖入栈顺序、默认容量、容量更新、删除、清空、图片文件清理、持久化关闭后不恢复。
- TypeScript 契约测试覆盖 command/event 名称和默认设置。
- `cargo test --manifest-path src-tauri/Cargo.toml` 和 `npm run test -w packages/shared-contracts` 作为本单元核心检查。
