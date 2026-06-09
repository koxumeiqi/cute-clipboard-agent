# Cute Clipboard Agent - Codex 项目上下文

林奕daddy，上下文已生效。

## 响应语言

- 默认使用简体中文回复。
- 用户明确要求其他语言时，按用户要求执行。

## 产品定位

- 面向 Windows 桌面的桌宠型剪贴板助手。
- MVP 核心闭环：桌宠常驻桌面，监听文本、emoji、图片剪贴板，保存最近 20 条历史，双击桌宠打开历史面板，点击历史项恢复到系统剪贴板。
- 必须支持暂停记录、清空历史、本地隐私告知。
- 坚持本地优先、用户可控、事件驱动。
- Agent Bridge 第一版只预留事件总线、权限模型和空实现，不接真实模型，不调用 OpenAI API、本地模型或 MCP。

## 技术栈

- 前端：React、TypeScript。
- 构建工具：Vite。
- 状态管理：Zustand。
- 图标库：lucide-react。
- 桌面壳与原生桥接：Tauri。
- 原生能力：Rust。
- 本地数据库：SQLite。
- 目标平台：Windows。

## Codex 会话入口

- 本文件是项目级 Codex/Agents 上下文入口。每次在本仓库工作时，优先遵守本文件。
- 更完整的目录说明见 `docs/architecture/project-structure.md`。
- 更完整的开发规范见 `docs/dev/development-rules.md`。
- 本机开发环境和常用命令见 `docs/development-environment.md`。
- PRD 驱动开发流程使用项目级 skill：`.codex/skills/prd-to-pr-workflow`。
- 当用户要求“按 PRD 开发”“实现某个 PRD”“执行 PRD workflow”“从需求到 PR”时，使用 `$prd-to-pr-workflow`。
- 如果用户提供的新指令与本文冲突，以用户当前会话中的最新明确指令为准。

## 工程目录

- `apps/desktop/`：React/TypeScript 前端应用。
- `apps/desktop/src/app/`：前端应用启动、路由/窗口级组合、全局 provider。
- `apps/desktop/src/windows/`：Tauri 多窗口前端入口，包括 `pet`、`history`、`settings`、`onboarding`。
- `apps/desktop/src/features/pet/`：桌宠 UI、拖拽、双击、右键菜单、待机动画。
- `apps/desktop/src/features/history-panel/`：剪贴板历史面板、历史项预览、恢复、删除、清空。
- `apps/desktop/src/features/settings/`：设置页、用户偏好、暂停记录、开机启动开关。
- `apps/desktop/src/features/privacy/`：首次告知、隐私状态、隐私控制入口。
- `apps/desktop/src/features/agent-panel/`：Agent 对话面板预留，MVP 不实现真实 AI。
- `apps/desktop/src/shared/`：前端共享 API、事件、类型、UI 基础组件和工具。
- `src-tauri/`：Tauri 与 Rust 原生应用。
- `src-tauri/src/commands/`：Tauri command 边界，只做参数校验、调用服务、返回 DTO。
- `src-tauri/src/clipboard/`：Windows 剪贴板监听、读取、写回抑制、内容归一化。
- `src-tauri/src/history/`：历史栈、容量裁剪、查询、删除、清空。
- `src-tauri/src/storage/`：SQLite 连接、Repository、应用数据目录。
- `src-tauri/src/image/`：图片保存、缩略图、图片文件清理。
- `src-tauri/src/settings/`：配置读取、保存、变更通知。
- `src-tauri/src/privacy/`：首次告知接受状态、隐私边界、权限状态。
- `src-tauri/src/tray/`：系统托盘、托盘菜单、托盘命令。
- `src-tauri/src/windows/`：Tauri 窗口创建、显示、隐藏、定位、置顶。
- `src-tauri/src/events/`：Rust 侧应用事件总线和 Tauri 事件转发。
- `src-tauri/src/agent_bridge/`：Agent Bridge 空实现、权限分级、上下文索引预留。
- `src-tauri/src/lifecycle/`：启动、退出、后台常驻、开机启动协调。
- `packages/shared-contracts/`：前后端共享契约，包括事件、模型、commands、权限枚举。
- `docs/prd/`：产品 PRD 单元。
- `docs/architecture/`：架构设计和目录说明。
- `docs/dev/`：开发规范。
- `docs/release/`：打包、发布和可信分发说明。
- `scripts/`：开发、构建、发布辅助脚本。

## 模块边界

- 桌宠模块只负责入口和桌面交互，不直接读写剪贴板或历史数据。
- 剪贴板模块只负责监听、读取、写回和归一化，不负责 UI 渲染和最终持久化策略。
- 历史模块负责历史生命周期和容量裁剪，不负责系统剪贴板监听。
- 设置模块负责配置状态，不直接实现底层能力；底层模块响应设置变更。
- 隐私模块定义告知、授权和控制边界，清空和暂停由存储、设置、监听模块执行。
- 托盘模块负责命令入口和生命周期编排，不直接实现业务细节。
- 事件总线负责模块解耦；事件 payload 默认只放元信息和预览信息，不放完整文本或原图数据。
- Agent Bridge MVP 只能订阅事件并做空处理，默认无完整内容访问权限。

## 开发规范

- PRD 功能开发默认走 `$prd-to-pr-workflow`：读上下文和 PRD，检查工作区，建分支，产出 `design/<prd-slug>/base-*.md`、`ui-*.md`、`app-*.md` 和 `tasks/<prd-slug>.md`，然后按 TDD 编码、检查、review、E2E、人工验证 gate、PR gate 推进。
- 新功能先对齐 `docs/prd/` 中对应 PRD，再改工程。
- 共享实体优先在 `packages/shared-contracts/` 定义，再分别映射到前端和 Rust。
- TypeScript 使用显式领域类型，避免在功能模块之间传递裸 `any`。
- Rust 侧按领域模块组织 service/repository/DTO，不把业务逻辑堆进 Tauri command。
- Tauri command 是边界层，参数校验和错误映射可以在这里做，核心逻辑下沉到 Rust 领域模块。
- SQLite schema 通过 `src-tauri/migrations/` 管理，不在业务代码里临时拼建表结构。
- 剪贴板、隐私、Agent 相关变更必须优先考虑“不上传、不越权、不泄露完整内容”。
- UI 优先复用 `apps/desktop/src/shared/ui/`，但只有出现真实重复或稳定模式时再抽象。
- 文件和目录命名使用 kebab-case；TypeScript 类型、React 组件、Rust 类型使用 PascalCase；函数和变量使用 camelCase/snake_case，遵循语言惯例。
- 不在本项目根目录散放源码；前端进入 `apps/desktop`，Rust/Tauri 进入 `src-tauri`，共享契约进入 `packages/shared-contracts`。

## 测试与验收

- 事件总线、设置、历史栈、隐私权限默认值需要单元测试。
- 剪贴板监听、写回抑制、图片历史清理需要集成测试或手工验证记录。
- 前端交互完成后使用浏览器或 Tauri 本地窗口验证关键页面，不只看代码。
- 发布前必须验证 Windows 安装包或便携包可启动，并确认首次隐私告知、托盘退出、清空历史可用。

## GitHub 克隆偏好

- 克隆 GitHub 开源仓库时优先使用 SSH。
- 用户 GitHub SSH key：`C:/Users/myz03/.ssh/id_ed25519_github`。
- Windows 下给 SSH key 路径使用正斜杠。
- 推荐命令：

```powershell
$env:GIT_SSH_COMMAND='ssh -i C:/Users/myz03/.ssh/id_ed25519_github -o IdentitiesOnly=yes'
git clone git@github.com:OWNER/REPO.git
```
