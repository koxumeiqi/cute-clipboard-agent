# 剪贴板历史栈与本地存储 - 基础设计

## 来源
- PRD：`docs/prd/03-剪贴板历史栈与本地存储.md`
- 分支：当前工作区位于 `codex/prd-02-clipboard-listener-normalization`，因存在前序未提交改动，本单元在同一工作区增量实现并记录风险。
- 相关模块：`src-tauri/src/history`、`src-tauri/src/storage`、`src-tauri/src/image`、`src-tauri/src/commands`、`packages/shared-contracts`

## 目标

提供本地优先的剪贴板历史生命周期能力：新条目按时间倒序入栈，默认保留 20 条，支持容量调整、查询、删除、清空和持久化开关。图片历史删除和清空时必须清理关联文件路径。

## 非目标

不实现历史面板 UI、收藏/固定、云同步、敏感内容识别、自动周期清理，也不接入真实 AI 或远端模型。

## 共享领域模型

- `ClipboardItem`：历史保存的完整条目。文本条目保存 `text` 和 `preview`；图片条目保存 `imagePath` 与 `thumbnailPath`。
- `ClipboardEventItem`：事件广播条目，只包含 ID、类型、预览、hash、时间和图片路径，不携带完整文本。
- `HistorySettings`：历史容量和持久化开关，默认 `capacity=20`、`persistEnabled=true`。
- `ClipboardHistorySnapshot`：历史列表、设置和总数，供 UI 或调试命令消费。

## 公共契约

### 命令 / API

- `list_clipboard_history`：查询倒序历史列表。
- `get_clipboard_history_item`：按 ID 查询完整条目。
- `delete_clipboard_history_item`：删除单条历史，并清理图片文件。
- `clear_clipboard_history`：清空全部历史，并清理图片文件。
- `update_clipboard_history_settings`：更新容量或持久化开关，容量只允许 10、20、50。
- `debug_process_clipboard_text`：调试入口，新文本被归一化后进入历史栈。

### 事件

- `clipboard.created`：新条目成功进入历史栈后广播。
- `clipboard.deleted`：单条历史删除后广播，payload 只包含 ID 和时间。
- `history.cleared`：全部历史清空后广播，payload 包含删除数量和时间。

### 权限

MVP 不给 Agent Bridge 完整内容读取权限。事件 payload 不放完整文本或图片字节，完整内容仅通过显式 command 获取。

### 持久化

持久化开启时，SQLite 保存历史条目和历史设置；持久化关闭时仅保留运行期内存历史，且重启不恢复旧历史。

## UI-App 集成契约

### UI 调用

历史面板后续通过 `shared/api/clipboard-api.ts` 调用历史 commands，设置页通过 `update_clipboard_history_settings` 调整容量和持久化。

### 应用 / 原生响应

Rust command 返回 camelCase DTO；错误以稳定字符串返回，例如 `history_item_not_found`、`invalid_history_capacity`、`history_storage_failed`。

### 事件订阅

UI 可以订阅 `clipboard.created`、`clipboard.deleted`、`history.cleared` 刷新列表。事件只作为同步信号，不作为完整数据源。

### 状态同步

容量变化立即裁剪内存和 SQLite 中最旧条目；清空和删除后查询结果必须同步反映变化。

### 错误映射

读取、写入、清理文件失败统一映射为可展示但不泄露本地敏感内容的错误字符串。

### 权限边界

存储模块不主动上传数据，不把完整文本写入事件或日志。清空历史必须同时删除 SQLite 记录和可清理的图片文件。

## 集成边界

剪贴板模块负责归一化，不负责最终持久化策略；历史模块接收 `ClipboardItem` 后负责入栈、裁剪、查询和删除；图片模块只负责本地文件删除工具；storage 模块只负责 SQLite 路径和连接初始化。

## BDD 场景

### 场景：新条目进入栈顶
前置条件：历史为空。  
触发动作：剪贴板监听产生文本条目。  
预期结果：查询历史时该条目位于列表第一项，并发出 `clipboard.created`。

### 场景：容量裁剪
前置条件：容量为 20。  
触发动作：连续写入 21 条历史。  
预期结果：历史数量为 20，最旧条目被移除。

### 场景：删除图片历史
前置条件：图片历史关联本地原图和缩略图。  
触发动作：删除该历史。  
预期结果：查询不到该条目，原图和缩略图文件被删除。

### 场景：关闭持久化
前置条件：历史中已有持久化记录。  
触发动作：关闭持久化并创建新运行期历史。  
预期结果：新条目只存在内存；重新初始化历史服务后不恢复该条目。

## 成功标准

| 标准 | 来源 | 自动化证据 | 人工证据 |
|---|---|---|---|
| 新历史写入后位于列表顶部 | AC-STORE-01 | Rust 单元测试 | 无 |
| 默认历史数量不超过 20 条 | AC-STORE-02 | Rust 单元测试 | 无 |
| 容量为 10 时自动裁剪 | AC-STORE-03 | Rust 单元测试 | 无 |
| 删除单条后查询不到 | AC-STORE-04 | Rust 单元测试 | 无 |
| 清空后历史为空 | AC-STORE-05 | Rust 单元测试 | 手工通过调试 command 查询 |
| 图片文件随删除/清空被清理 | AC-STORE-06 | Rust 单元测试 | Windows 数据目录检查 |
| 关闭持久化后重启不恢复旧历史 | AC-STORE-07 | Rust 单元测试 | 重启应用检查 |

## 风险

- 当前上游图片归一化仍返回 `pending://` 路径，本单元先保证历史生命周期和真实文件路径清理能力，图片字节保存需在后续图片增强中接入。
- 工作区已有前序未提交改动，本单元无法安全切新分支。

## 未决问题

- 配置长期是否统一放 SQLite，还是设置模块继续使用 JSON，需要在 PRD 05 统一。
