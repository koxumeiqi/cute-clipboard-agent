# 项目目录结构

本文档只定义工程结构和模块边界，不包含业务代码实现。

## 顶层结构

```text
cute-clipboard-agent/
  apps/
    desktop/
  src-tauri/
  packages/
    shared-contracts/
  docs/
    prd/
    architecture/
    dev/
    release/
  scripts/
```

## 前端应用

```text
apps/desktop/src/
  app/
  assets/
  windows/
    pet/
    history/
    settings/
    onboarding/
  features/
    pet/
    history-panel/
    settings/
    privacy/
    agent-panel/
  shared/
    api/
    events/
    lib/
    types/
    ui/
```

- `app/`：前端应用组合层，放全局 provider、窗口入口组合、应用级状态协调。
- `windows/`：按 Tauri 窗口拆入口。桌宠窗口、历史窗口、设置窗口、首次告知窗口彼此独立。
- `features/pet/`：桌宠 UI、透明窗口内交互、拖拽状态、双击打开历史、右键菜单 UI。
- `features/history-panel/`：历史列表、文本/emoji/图片预览、恢复、删除、清空、空状态。
- `features/settings/`：历史容量、记录类型、持久化、暂停记录、动画、自动移动、开机启动设置。
- `features/privacy/`：首次告知、隐私文案、接受状态展示、清空/暂停/持久化隐私语义。
- `features/agent-panel/`：后续 Agent 对话面板预留；MVP 可以只有工程占位。
- `shared/api/`：封装 Tauri invoke 和事件监听，不让业务组件直接散落 bridge 调用。
- `shared/events/`：前端事件订阅、派发、事件名常量。
- `shared/types/`：前端 DTO 和视图模型类型。
- `shared/ui/`：按钮、开关、列表项、弹窗等可复用基础 UI。
- `shared/lib/`：纯函数工具。

## Rust/Tauri 应用

```text
src-tauri/
  capabilities/
  icons/
  migrations/
  src/
    app/
    commands/
    clipboard/
    history/
    settings/
    privacy/
    tray/
    windows/
    events/
    agent_bridge/
    storage/
    image/
    lifecycle/
    shared/
```

- `commands/`：Tauri command 注册与边界适配。
- `clipboard/`：Windows 剪贴板监听、文本/emoji/图片读取、写回抑制、归一化。
- `history/`：历史栈、容量裁剪、查询、删除、清空。
- `storage/`：SQLite 连接、事务、Repository、应用数据目录定位。
- `image/`：图片文件保存、缩略图生成、图片历史删除清理。
- `settings/`：`AppSettings` 读取、保存、变更事件。
- `privacy/`：`PrivacyState`、首次告知接受状态、Agent 默认权限边界。
- `tray/`：系统托盘图标、菜单、命令分发。
- `windows/`：桌宠、历史、设置、首次告知窗口的创建、显示、隐藏、定位、置顶。
- `events/`：Rust 侧事件总线，负责内部订阅和向前端转发必要事件。
- `agent_bridge/`：Agent Bridge 空处理器、权限分级、上下文结构预留。
- `lifecycle/`：应用启动、退出、后台常驻、开机启动协调。
- `shared/`：Rust 通用错误、时间、ID、序列化辅助。

## 共享契约

```text
packages/shared-contracts/src/
  models/
  events/
  commands/
  permissions/
```

- `models/`：`ClipboardItem`、`AppSettings`、`PrivacyState`、`AgentContext` 等共享实体。
- `events/`：`AppEvent`、剪贴板、历史、桌宠、设置、隐私、Agent 事件契约。
- `commands/`：Tauri command 的入参、出参契约。
- `permissions/`：`AgentPermissionLevel` 和内容访问分级。

共享契约是前后端对齐的源头。实现阶段如果 Rust 和 TypeScript 需要分别生成类型，必须保持字段命名、事件名和权限语义一致。

## PRD 到工程模块映射

| PRD | 前端目录 | Rust/Tauri 目录 | 共享契约 |
|---|---|---|---|
| 01 桌宠入口与桌面交互 | `features/pet`、`windows/pet` | `windows`、`events`、`settings` | `events`、`models` |
| 02 剪贴板监听与内容归一化 | `shared/events` | `clipboard`、`image`、`events` | `models`、`events` |
| 03 剪贴板历史栈与本地存储 | `shared/api` | `history`、`storage`、`image` | `models`、`commands`、`events` |
| 04 剪贴板历史面板 | `features/history-panel`、`windows/history` | `commands`、`history`、`clipboard` | `models`、`commands`、`events` |
| 05 设置与用户偏好 | `features/settings`、`windows/settings` | `settings`、`lifecycle` | `models`、`events`、`commands` |
| 06 系统托盘与应用生命周期 | `shared/events` | `tray`、`lifecycle`、`windows` | `events`、`commands` |
| 07 隐私安全与首次告知 | `features/privacy`、`windows/onboarding` | `privacy`、`settings`、`history` | `models`、`permissions`、`events` |
| 08 事件总线与智能体桥接预留 | `shared/events`、`features/agent-panel` | `events`、`agent_bridge` | `events`、`permissions`、`models` |
| 09 视窗打包与分发 | 无直接页面，依赖首次告知 | `capabilities`、`icons`、`lifecycle` | 发布配置不进入共享契约 |

