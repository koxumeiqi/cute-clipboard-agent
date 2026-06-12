# 剪贴板监听与内容归一化 - 应用设计

## 运行时职责
- `clipboard` 模块维护记录设置、最近 hash、写回抑制队列。
- 监听到系统变化后读取原始内容，归一化为 `ClipboardItem` 或忽略原因。
- 通过事件模块发布 `clipboard.changed`、`clipboard.created`、`clipboard.duplicated`、`clipboard.read_failed`。

## 模块变更
- `src-tauri/src/clipboard/mod.rs`：新增领域模型、设置存储、归一化服务和测试。
- `src-tauri/src/events/mod.rs`：新增剪贴板事件发射函数。
- `src-tauri/src/commands/mod.rs`：新增剪贴板设置、暂停、写回抑制命令。
- `packages/shared-contracts`：新增模型、事件、命令契约和测试。
- `apps/desktop/src/shared/api/clipboard-api.ts`：新增前端命令封装。

## 数据流
1. 系统剪贴板变化触发读取。
2. 读取结果转为 `ClipboardRawContent`。
3. `ClipboardRecorder` 检查暂停和类型开关。
4. 计算内容 hash。
5. 检查写回抑制和栈顶重复。
6. 生成 `ClipboardItem` 并发出创建事件，或发出忽略/失败事件。

## 存储 / Schema
- 不新增数据库 schema。
- `ClipboardRecordingSettings` 使用本地 JSON 文件保存，后续 PRD 05 可统一迁移设置中心。

## 错误处理
- 空内容、类型不支持、暂停、禁用类型、重复、写回抑制都不抛 UI 错误，只返回 `ClipboardProcessOutcome` 并发事件。
- 设置读写失败映射为字符串错误码。

## 隐私与安全
- 所有处理本地完成。
- 事件 payload 不携带完整图片字节。
- Agent Bridge 只可订阅元信息事件，默认不读取完整内容。

## CDD 契约
### 输入契约
- `ClipboardRawContent::Text { text }`
- `ClipboardRawContent::Image { bytes, extension, width, height }`
- 设置更新 DTO 仅允许布尔开关。

### 输出契约
- `ClipboardItem` 使用 camelCase 序列化给前端。
- `ClipboardProcessOutcome` 内部分为 `Created`、`Duplicate`、`Ignored`。

### 事件契约
- `clipboard.created`：`{ at, item: ClipboardEventItem }`
- `clipboard.duplicated`：`{ at, hash }`
- `clipboard.read_failed`：`{ at, reason }`
- `clipboard.changed`：`{ at }`

### 权限契约
- 无网络权限。
- 不向事件总线广播完整文本字段和原图字节。

### 错误契约
- `invalid_clipboard_settings`
- `clipboard_settings_read_failed`
- `clipboard_settings_write_failed`
- `clipboard_read_failed`

### 持久化契约
- 设置 JSON 缺失时使用默认值：`paused=false`、`recordText=true`、`recordImage=true`。

## 测试策略
- 共享契约测试：事件名、命令名、默认设置。
- Rust 领域测试：默认设置、文本/emoji 归一化、空内容忽略、暂停忽略、类型开关、去重、写回抑制、图片条目。
- Rust 集成边界：命令可编译，事件 payload 可序列化。
