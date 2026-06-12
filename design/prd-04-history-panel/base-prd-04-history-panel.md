# 剪贴板历史面板 - 基础设计

## 来源
- PRD：`docs/prd/04-剪贴板历史面板.md`
- 分支：`codex/prd-04-history-panel`
- 相关模块：`apps/desktop/src/features/history-panel`、`apps/desktop/src/windows/history`、`apps/desktop/src/shared/api`、`packages/shared-contracts`、`src-tauri/src/commands`、`src-tauri/src/history`、`src-tauri/src/clipboard`

## 目标
- 用户双击桌宠后可以打开历史面板窗口。
- 面板按最近在前展示剪贴板历史，支持文本、emoji 与图片缩略图预览。
- 用户点击历史项可恢复到系统剪贴板，删除按钮可删除单条，底部操作可清空全部。
- 面板具备加载、空状态、错误与恢复成功反馈。

## 非目标
- 不实现搜索、筛选、收藏、固定历史、自然语言检索。
- 不在历史面板加入 Agent 操作按钮。
- 不改变历史存储容量策略与持久化策略。

## 共享领域模型
- `ClipboardItem`：历史项，包含 `id`、`type`、`preview`、`text`、`imagePath`、`thumbnailPath`、`hash`、`createdAt`、`updatedAt`。
- `ClipboardHistorySnapshot`：历史快照，包含 `items`、`settings`、`total`。
- `ClipboardHistoryItemRequest`：按 `id` 操作历史项。

## 公共契约

### 命令 / API
- `list_clipboard_history`：返回当前历史快照。
- `get_clipboard_history_item`：按 id 返回完整历史项。
- `restore_clipboard_history_item`：按 id 写回系统剪贴板。
- `delete_clipboard_history_item`：删除单条并返回最新快照。
- `clear_clipboard_history`：清空全部并返回删除数量。
- `open_history_panel` / `close_history_panel`：打开与关闭历史窗口。

### 事件
- `clipboard.deleted`：删除单条后只携带 id。
- `history.cleared`：清空后携带删除数量。
- 恢复操作通过写回抑制避免再次入栈，不新增携带完整内容的事件。

### 权限
- 历史面板属于用户主动操作的本地 UI，有权通过 Tauri command 读取完整历史项用于展示和恢复。
- Agent Bridge 不因此获得完整剪贴板内容访问权限。

### 持久化
- 复用 PRD 03 的 SQLite 历史存储与图片文件清理规则。
- 本 PRD 不新增 schema。

## UI-App 集成契约

### UI 调用
- `HistoryWindow` 初始化调用 `listClipboardHistory()`。
- 点击历史项调用 `restoreClipboardHistoryItem({ id })`。
- 点击删除按钮调用 `deleteClipboardHistoryItem({ id })`，并阻止触发恢复。
- 点击清空调用 `clearClipboardHistory()` 后刷新列表。

### 应用 / 原生响应
- 列表命令返回最新在前的 `items`。
- 文本恢复写回 `item.text`。
- 图片恢复读取 `item.imagePath` 并写回系统剪贴板图片。
- 找不到条目或写回失败时返回稳定错误字符串。

### 事件订阅
- MVP 面板打开后主动刷新，不依赖实时订阅。
- 后续可订阅 `clipboard.created`、`clipboard.deleted`、`history.cleared` 做自动刷新。

### 状态同步
- `loading`：初始加载或刷新中。
- `ready`：列表可用。
- `empty`：`ready` 且 `items.length === 0`。
- `restoring`：单条恢复中。
- `error`：命令失败，保留当前可见列表。

### 错误映射
- `history_item_not_found`：提示记录不存在或已被删除。
- `clipboard_write_failed`：提示写回失败。
- `unsupported_clipboard_restore`：提示该内容暂不支持恢复。
- 其他错误：提示稍后重试。

### 权限边界
- 面板仅在本地窗口展示历史内容，不上传、不记录到日志。
- 删除和清空通过 history 模块执行，图片文件清理仍由存储/图片模块负责。

## 集成边界
- 桌宠只负责触发打开历史窗口，不直接读写历史。
- 历史面板只调用共享 API，不直接调用 Tauri `invoke`。
- Tauri command 只做边界适配，历史生命周期交给 `history` 模块，写回抑制交给 `clipboard` 模块。

## BDD 场景

### 场景：双击桌宠打开历史面板
前置条件：桌宠窗口已显示。  
触发动作：用户双击桌宠。  
预期结果：历史窗口打开并开始读取历史。

### 场景：展示最近历史
前置条件：历史栈中有多条记录。  
触发动作：用户打开历史面板。  
预期结果：列表按创建时间倒序展示，最新内容在顶部。

### 场景：恢复文本或 emoji
前置条件：历史列表中存在文本或 emoji 记录。  
触发动作：用户点击该历史项。  
预期结果：系统剪贴板变为该记录内容，并显示恢复成功反馈。

### 场景：恢复图片
前置条件：历史列表中存在带 `imagePath` 的图片记录。  
触发动作：用户点击该图片历史项。  
预期结果：系统剪贴板变为该图片，并显示恢复成功反馈。

### 场景：删除单条历史
前置条件：历史列表中存在记录。  
触发动作：用户点击该记录的删除按钮。  
预期结果：该记录从列表消失，且不会触发恢复。

### 场景：清空历史
前置条件：历史列表中存在记录。  
触发动作：用户点击清空历史。  
预期结果：列表进入空状态。

## 成功标准
| 标准 | 来源 | 自动化证据 | 人工证据 |
|---|---|---|---|
| 双击桌宠后历史面板打开 | AC-PANEL-01 | 前端组件测试覆盖入口调用 | Tauri 桌面窗口手工验证 |
| 历史列表按时间倒序展示 | AC-PANEL-02 | history store 单元测试 | 面板列表目视确认 |
| 文本和 emoji 正确预览 | AC-PANEL-03 | history 面板组件测试 | 面板目视确认 |
| 图片历史显示缩略图 | AC-PANEL-04 | history 面板组件测试 | 面板目视确认 |
| 点击文本历史写回剪贴板 | AC-PANEL-05 | Tauri command 测试或人工验证 | Windows 剪贴板验证 |
| 点击图片历史写回剪贴板 | AC-PANEL-06 | Rust 单元测试覆盖读取图片路径 | Windows 剪贴板验证 |
| 删除单条后列表更新 | AC-PANEL-07 | history 面板组件测试 | 面板目视确认 |
| 清空后进入空状态 | AC-PANEL-08 | history 面板组件测试 | 面板目视确认 |

## 风险
- Windows 图片剪贴板写回依赖 `arboard` 对图片格式的支持，需要真实环境验证。
- 现有仓库存在大量前序未跟踪文件，本次改动需避免清理或覆盖无关产物。

## 未决问题
- MVP 清空历史是否需要二次确认：按 PRD 当前约定直接清空。
