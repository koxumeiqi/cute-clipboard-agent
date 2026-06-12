# 剪贴板监听与内容归一化 - 基础设计

## 来源
- PRD：`docs/prd/02-剪贴板监听与内容归一化.md`
- 分支：`codex/prd-02-clipboard-listener-normalization`
- 相关模块：`src-tauri/src/clipboard`、`src-tauri/src/settings`、`src-tauri/src/events`、`src-tauri/src/commands`、`packages/shared-contracts`

## 目标
- 将系统剪贴板变化读取为统一 `ClipboardItem`。
- 支持文本、emoji 和图片三类 MVP 内容。
- 支持暂停记录、文本/图片类型开关、栈顶去重、写回抑制和读取失败事件。
- 事件 payload 默认只携带元信息和预览信息，不向 UI/Agent 广播完整文本或原图字节。

## 非目标
- 不做 HTML、URL、文件路径、代码片段分类。
- 不做 OCR、图片理解、云同步或 Agent 分析。
- 不在监听模块内决定最终 SQLite 持久化策略。

## 共享领域模型
- `ClipboardItem`：统一条目，包含 `id`、`type`、`preview`、可选 `text`、`imagePath`、`thumbnailPath`、`hash`、`createdAt`、`updatedAt`。
- `ClipboardItemType`：`text` 或 `image`。
- `ClipboardRecordingSettings`：`paused`、`recordText`、`recordImage`。
- `ClipboardReadFailureReason`：`empty`、`unsupportedType`、`readFailed`、`recordingPaused`、`typeDisabled`、`duplicate`、`selfWriteSuppressed`。

## 公共契约
### 命令 / API
- `get_clipboard_recording_settings`：读取记录设置。
- `update_clipboard_recording_settings`：局部更新 `recordText`、`recordImage`。
- `set_recording_paused`：设置暂停记录状态。
- `suppress_next_clipboard_hash`：历史面板写回前登记下一次 hash，避免自触发重复记录。

### 事件
- `clipboard.changed`：监听到系统变化，只携带时间。
- `clipboard.created`：归一化成功，只携带条目元信息、预览、hash，不携带原图字节。
- `clipboard.duplicated`：栈顶重复被忽略。
- `clipboard.read_failed`：空内容、禁用类型、读取失败等原因。

### 权限
- 监听模块只读取本机剪贴板，不访问网络。
- Agent Bridge MVP 不获得完整内容访问权限。
- 完整文本只进入本地历史/存储链路，不通过事件广播给非必要订阅方。

### 持久化
- 本 PRD 不新增 SQLite schema。
- 记录设置先复用轻量本地 JSON settings；历史持久化由 PRD 03 接入。

## UI-App 集成契约
### UI 调用
- 设置页后续通过命令读取和更新暂停、文本记录、图片记录开关。
- 历史面板恢复内容前调用写回抑制命令。

### 应用 / 原生响应
- 原生层返回更新后的 `ClipboardRecordingSettings`。
- 读取失败返回稳定错误码字符串，不弹窗打断用户。

### 事件订阅
- 历史模块订阅 `clipboard.created` 后再决定是否持久化。
- UI 可订阅 `clipboard.read_failed` 用于诊断，不强制展示。

### 状态同步
- `set_recording_paused` 与 `update_clipboard_recording_settings` 都更新同一设置源。

### 错误映射
- 非法输入：`invalid_clipboard_settings`
- 设置读写失败：`clipboard_settings_read_failed`、`clipboard_settings_write_failed`
- 读取失败事件原因使用枚举字符串。

### 权限边界
- 事件中不携带完整图片字节。
- `clipboard.created` 的文本内容只暴露预览；完整文本由后续本地历史服务内部消费。

## 集成边界
- 剪贴板模块负责监听、读取、归一化、过滤和抑制。
- 历史模块负责容量裁剪和持久化。
- 图片模块负责后续图片文件和缩略图落盘。
- 设置模块负责用户偏好存储。

## BDD 场景
### 场景：复制文本生成条目
前置条件：记录未暂停，文本记录开启。  
触发动作：系统剪贴板出现非空文本。  
预期结果：生成 `type=text` 的 `ClipboardItem`，包含 preview 和 hash，并发布 `clipboard.created`。

### 场景：复制 emoji 按文本处理
前置条件：记录未暂停，文本记录开启。  
触发动作：系统剪贴板出现 emoji 字符串。  
预期结果：生成文本条目，preview 保留原 emoji 字符。

### 场景：暂停记录时忽略变化
前置条件：`paused=true`。  
触发动作：系统剪贴板出现文本或图片。  
预期结果：不生成条目，发布 `clipboard.read_failed`，原因为 `recordingPaused`。

### 场景：连续相同内容去重
前置条件：上一条 hash 与本次内容 hash 相同。  
触发动作：系统剪贴板再次出现相同内容。  
预期结果：不生成条目，发布 `clipboard.duplicated`。

### 场景：写回抑制
前置条件：历史面板恢复内容前登记了对应 hash。  
触发动作：系统剪贴板变化为该 hash 内容。  
预期结果：不生成新条目，发布 `clipboard.read_failed`，原因为 `selfWriteSuppressed`。

## 成功标准
| 标准 | 来源 | 自动化证据 | 人工证据 |
|---|---|---|---|
| 文本复制可生成文本条目 | AC-CLIP-01 | Rust 领域测试 | Windows 复制文本后历史新增 |
| emoji 保留为文本预览 | AC-CLIP-02 | Rust 领域测试 | Windows 复制 emoji 后预览正确 |
| 图片复制可生成图片条目 | AC-CLIP-03 | Rust 领域测试覆盖归一化，真实读取人工验证 | Windows 复制图片后历史新增 |
| 暂停记录不新增历史 | AC-CLIP-04 | Rust 领域测试 | 设置暂停后复制不新增 |
| 禁用图片记录后忽略图片 | AC-CLIP-05 | Rust 领域测试 | 关闭图片记录后复制图片不新增 |
| 相同内容不重复入栈 | AC-CLIP-06 | Rust 领域测试 | 连续复制相同内容只有一条 |
| 写回不会重复记录 | AC-CLIP-07 | Rust 领域测试 | 历史恢复后不新增重复项 |

## 风险
- 当前自动化环境不能可靠触发 Windows 全局剪贴板消息，真实监听需要 Windows 桌面环境人工验证。
- 图片落盘和缩略图由后续存储/图片模块完成，本单元只定义路径字段和归一化边界。

## 未决问题
- 真实图片文件保存路径和缩略图策略在 PRD 03/图片模块中最终确定。
