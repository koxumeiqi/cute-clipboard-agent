# 剪贴板监听与内容归一化 - 任务计划

## 状态
- 分支：`codex/prd-02-clipboard-listener-normalization`
- PRD：`docs/prd/02-剪贴板监听与内容归一化.md`
- 设计：`design/prd-02-clipboard-listener-normalization/`

## 计划
- [x] 1. 根据 PRD/design 定义实现切片
- [x] 2. 运行设计产物 review loop
- [x] 3. 为 slice 1 创建失败测试
- [x] 4. 实现 slice 1 直到测试通过
- [x] 5. 对剩余 slices 重复 Red/Green/Refactor
- [x] 6. 串联集成边界
- [x] 7. 运行检查
- [x] 8. 运行代码 review 修复 loop
- [x] 9. 运行 E2E 验证
- [ ] 10. 完成人工验证 gate
- [ ] 11. 准备或创建 PR

## 设计 Review 记录
| 轮次 | Reviewer | 结论 | 偏离/阻塞问题 | 调整结果 | 是否继续 review |
|---|---|---|---|---|---|
| 1 | Codex 内部 fallback | 无阻塞偏离。文档覆盖 P0/P1、非目标、BDD/CDD、事件隐私边界和人工验证缺口。 | 真实 Windows 全局剪贴板监听无法在当前自动化环境完整验证。 | 已记录为风险和人工验证项。 | 否 |

## 实现切片
| 切片 | 行为 | 是否测试先行 | 负责区域 | 状态 |
|---|---|---|---|---|
| 1 | 共享契约定义剪贴板模型、命令、事件和默认设置 | 是 | `packages/shared-contracts` | 已完成 |
| 2 | Rust 领域归一化：文本、emoji、图片、hash、preview | 是 | `src-tauri/src/clipboard` | 已完成 |
| 3 | 过滤与状态：暂停、类型开关、空内容、重复、写回抑制 | 是 | `src-tauri/src/clipboard` | 已完成 |
| 4 | 命令与事件接入 Tauri 边界 | 是 | `src-tauri/src/commands`、`src-tauri/src/events` | 已完成 |
| 5 | 前端 API 封装 | 否，契约测试覆盖命令名 | `apps/desktop/src/shared/api` | 已完成 |

## TDD 记录
| 切片 | Red 命令 / 结果 | Green 命令 / 结果 | Refactor 说明 | 阻塞项 / 替代验证 |
|---|---|---|---|---|
| 1 | `npm run test -w packages/shared-contracts` 初始新增测试期望剪贴板命令、事件、默认设置和事件脱敏契约存在。 | `npm run test -w packages/shared-contracts` 通过，4 个测试文件 16 个用例通过。 | 无。 | 无 |
| 2 | `cargo test --manifest-path src-tauri/Cargo.toml` 初始新增 Rust 领域测试期望归一化和过滤行为存在。 | `cargo test --manifest-path src-tauri/Cargo.toml` 通过，15 个 Rust 用例通过。 | 无。 | 无 |
| 3 | `cargo test --manifest-path src-tauri/Cargo.toml` 覆盖暂停、禁用类型、重复、写回抑制。 | `npm run test` 通过，shared contracts、desktop、Rust 全部通过。 | 无。 | 无 |
| 4 | 命令/事件编译由 `cargo check --manifest-path src-tauri/Cargo.toml` 验证。 | `cargo check --manifest-path src-tauri/Cargo.toml` 通过。 | 使用 `arboard` 启动后台轮询监听，真实系统剪贴板操作仍需 Windows 人工验证。 | 自动化覆盖领域和编译边界 |
| 5 | 前端 API 不单独新增 UI 测试，命令名由共享契约测试覆盖。 | `npm run build` 通过。 | 无。 | 无 |

## 检查结果
- `npm run test`：通过。shared contracts 4 个测试文件 16 个用例，desktop 1 个测试文件 6 个用例，Rust 15 个用例通过。
- `npm run build`：通过。shared contracts TypeScript 编译和 desktop Vite build 通过。
- `cargo check --manifest-path src-tauri/Cargo.toml`：通过。
- `cargo fmt --manifest-path src-tauri/Cargo.toml`：已执行。首次失败原因是 toolchain 缺少 rustfmt，已通过 `rustup component add rustfmt` 安装后成功。

## Review 结果
| 轮次 | Reviewer | 结论 | 阻塞问题 | 修复结果 | 是否继续 review |
|---|---|---|---|---|---|
| 1 | Codex 内部 fallback | 未发现 P0/P1 阻塞问题。实现覆盖 PRD 的文本/emoji/图片归一化、暂停、类型开关、去重、写回抑制和事件脱敏；测试覆盖核心领域规则。 | 无。 | 无需修复。 | 否 |

## E2E 结果
自动化 E2E 未启动真实 Tauri 窗口。当前验证使用 Rust 领域测试、Tauri 编译检查、前端构建和共享契约测试覆盖可自动化部分。真实 Windows 剪贴板监听、图片读取和历史面板入栈仍需要完整应用人工验证。

## 人工验证
- 复制普通文本，历史新增文本记录。
- 复制 emoji，预览保留 emoji。
- 复制图片，历史新增图片记录。
- 暂停记录后复制内容，历史不新增。
- 关闭图片记录后复制图片，历史不新增。
- 从历史恢复内容，不新增重复记录。

## PR 就绪度
- [x] 设计文档完整
- [x] 设计产物 review loop 完成，且无阻塞 PRD 偏离
- [x] TDD 记录完整，或 blocker 已记录
- [x] 检查完成，或 blocker 已记录
- [x] 代码 review 修复 loop 完成
- [x] 阻塞 review 问题已解决或被接受
- [x] E2E 完成，或缺口已记录
- [ ] 人工验证已通过或被明确豁免
- [ ] 如需创建 PR，分支已 commit 并 push
