# 开发环境说明

## 已安装工具

这台 Windows 电脑已经为 Cute Clipboard Agent 项目准备好了 Tauri、React、TypeScript、Rust 和 SQLite 相关开发环境。

已验证版本：

```txt
Node.js: v24.16.0
npm: 11.13.0
Rust: rustc 1.96.0
Cargo: cargo 1.96.0
Tauri CLI: 2.11.2，可通过 npx 使用
Git: 已安装
Microsoft Edge WebView2 Runtime: 149.0.4022.52
Visual Studio Build Tools: 2022 BuildTools，已包含 C++ 工具链
MSVC Toolset: 14.44.35207
```

## 项目技术栈

```txt
桌面框架：Tauri
前端框架：React
前端语言：TypeScript
构建工具：Vite
原生能力：Rust
本地数据库：SQLite
状态管理：Zustand
图标库：lucide-react
打包发布：Tauri Bundler
```

## 环境说明

- PowerShell 当前用户执行策略已设置为 `RemoteSigned`，因此可以正常执行 `npm`。
- Rust 通过 `rustup` 安装。
- Rust 工具链为 `stable-x86_64-pc-windows-msvc`。
- Windows 上开发 Tauri/Rust 原生应用需要 Microsoft C++ Build Tools。
- Windows 上运行 Tauri 应用需要 Microsoft Edge WebView2 Runtime。
- Tauri CLI 不需要全局安装，当前可通过 `npx @tauri-apps/cli` 使用。

## 常用命令

创建 Tauri React TypeScript 项目：

```powershell
npm create tauri-app@latest
```

检查 Tauri CLI：

```powershell
npx @tauri-apps/cli --version
```

项目初始化后运行开发服务：

```powershell
npm install
npm run tauri dev
```

构建 Windows 安装包：

```powershell
npm run tauri build
```
