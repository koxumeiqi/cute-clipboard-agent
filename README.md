# Cute Clipboard Agent

Cute Clipboard Agent 是一个面向 Windows 桌面的桌宠型剪贴板助手。项目目标是让桌宠常驻桌面，自动记录本地剪贴板内容，并通过轻量历史面板快速恢复最近使用过的文本、emoji 和图片。

当前实现坚持本地优先、用户可控和事件驱动。剪贴板内容默认只在本机处理，Agent Bridge 相关能力仅预留结构，不接入真实模型，也不调用 OpenAI API、本地模型或 MCP。

## 目前支持的功能

- 桌宠入口：提供独立桌宠窗口，支持常驻桌面、置顶配置、待机动画和可拖拽移动。
- 桌宠交互：双击或键盘 Enter/Space 打开剪贴板历史面板，右键打开菜单。
- 右键菜单：提供打开历史、打开设置、暂停记录、退出应用等入口。
- 剪贴板监听：Rust/Tauri 侧通过轮询监听系统剪贴板，支持文本和图片读取。
- 内容归一化：对文本和图片生成 hash，过滤空内容和重复内容，并支持写回抑制，避免恢复历史项时被重复记录。
- 历史栈：保存最近 20 条剪贴板记录，支持容量裁剪、查询、删除、清空和恢复。
- 历史面板：展示最近记录、文本预览、图片缩略图或图片占位图、记录时间、恢复状态和空/加载/失败状态。
- 恢复到剪贴板：点击历史项可把对应文本或图片写回系统剪贴板。
- 暂停记录：提供记录暂停状态和更新命令，底层监听会根据设置跳过记录。
- 本地存储：使用 SQLite 保存剪贴板历史，图片内容保存为本地文件并维护缩略图路径。
- 应用窗口：包含桌宠窗口、历史窗口和设置窗口的 Tauri 窗口创建、显示、隐藏、定位与关闭处理。
- 事件总线：预留并实现剪贴板变更、创建、重复、读取失败、删除、清空，以及桌宠双击、拖拽结束等事件转发。
- 共享契约：通过 `packages/shared-contracts` 维护前后端共享的 commands、events、models 和权限枚举基础结构。
- 自动化测试：已覆盖共享契约、桌宠交互、历史面板交互，以及 Rust 侧剪贴板、历史、设置等核心逻辑。

## 技术栈

- 前端：React、TypeScript、Vite、Zustand、lucide-react。
- 桌面壳：Tauri 2。
- 原生能力：Rust。
- 剪贴板：arboard。
- 本地数据库：SQLite / rusqlite。
- 目标平台：Windows。

## 项目结构

```text
apps/desktop/                 React + Vite 桌面前端
apps/desktop/src/windows/     pet、history、settings、onboarding 等窗口入口
apps/desktop/src/features/    桌宠、历史面板、设置、隐私、Agent 预留等功能模块
apps/desktop/src/shared/      前端共享 API、事件、类型和 UI
packages/shared-contracts/    前后端共享契约
src-tauri/                    Tauri 与 Rust 原生应用
src-tauri/src/clipboard/      剪贴板监听、读取、写回抑制和归一化
src-tauri/src/history/        历史栈、容量裁剪、查询、删除、清空
src-tauri/src/storage/        SQLite 连接和应用数据目录
src-tauri/src/windows/        桌面窗口创建、显示、隐藏、定位
docs/prd/                     PRD 单元
design/                       PRD 开发设计产物
tasks/                        PRD 开发任务清单
scripts/                      开发、诊断和 E2E 辅助脚本
```

更完整的目录说明见 `docs/architecture/project-structure.md`。

## 本地开发

安装依赖：

```powershell
npm install
```

启动前端开发服务：

```powershell
npm run dev
```

启动 Tauri 开发窗口：

```powershell
npm run tauri:dev
```

构建前端和共享契约：

```powershell
npm run build
```

运行测试：

```powershell
npm test
```

## 当前边界

- MVP 阶段不上传剪贴板内容，不接真实 AI 模型，不调用外部模型 API。
- Agent Bridge 仅保留事件总线、权限模型和空实现方向。
- 设置、隐私告知、托盘生命周期和打包分发仍按 PRD 单元继续推进。
