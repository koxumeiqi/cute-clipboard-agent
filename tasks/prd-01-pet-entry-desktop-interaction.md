# PRD01 桌宠入口与桌面交互 - Tasks

## 状态
- Branch: `codex/prd-01-pet-entry-desktop-interaction`
- PRD: `docs/prd/01-桌宠入口与桌面交互.md`
- Design: `design/prd-01-pet-entry-desktop-interaction/`

## 计划
- [x] 1. 根据 PRD/design 定义实现 slices
- [x] 2. 运行设计产物 review loop
- [ ] 3. 为 slice 1 创建失败测试
- [ ] 4. 实现 slice 1 直到测试通过
- [ ] 5. 对剩余 slices 重复 Red/Green/Refactor
- [ ] 6. 串联集成边界
- [ ] 7. 运行检查
- [ ] 8. 运行代码 review 修复 loop
- [ ] 9. 运行 E2E 验证
- [ ] 10. 完成人工验证 gate
- [ ] 11. 准备或创建 PR

## 设计 Review 记录
| 轮次 | Reviewer | 结论 | 偏离/阻塞问题 | 调整结果 | 是否继续 review |
|---|---|---|---|---|---|
| 1 | Internal fallback | 通过 | 无阻塞；仓库是骨架工程，需要在任务中记录基础工程初始化 | 已在 base/app 风险与实现 slice 中记录 | 否 |

## 实现 Slices
| Slice | 行为 | 是否测试先行 | 负责区域 | 状态 |
|---|---|---|---|---|
| 1 | 共享契约定义 pet settings、events、commands | 是 | `packages/shared-contracts` | 待执行 |
| 2 | React pet UI 支持加载、双击、右键、idle animation 状态 | 是 | `apps/desktop` | 待执行 |
| 3 | Rust settings persistence 和 Tauri commands | 是 | `src-tauri` | 待执行 |
| 4 | Tauri 窗口配置与历史/设置窗口占位 | 替代验证 | `src-tauri`, `apps/desktop` | 待执行 |

## TDD 记录
| Slice | Red 命令 / 结果 | Green 命令 / 结果 | Refactor 说明 | Blocker / 替代验证 |
|---|---|---|---|---|
|  |  |  |  |  |

## 检查结果
待执行。

## Review 结果
| 轮次 | Reviewer | 结论 | 阻塞问题 | 修复结果 | 是否继续 review |
|---|---|---|---|---|---|
|  |  |  |  |  |  |

## E2E 结果
待执行。

## 人工验证
真实 Windows 桌面窗口行为需要人工验证：透明无边框、拖拽跟手、重启恢复位置、双击打开历史、右键菜单。

## PR 就绪度
- [x] 设计文档完整
- [x] 设计产物 review loop 完成，且无阻塞 PRD 偏离
- [ ] TDD 记录完整，或 blocker 已记录
- [ ] 检查完成，或 blocker 已记录
- [ ] 代码 review 修复 loop 完成
- [ ] 阻塞 review 问题已解决或被接受
- [ ] E2E 完成，或缺口已记录
- [ ] 人工验证已通过或被明确豁免
- [ ] 如需创建 PR，分支已 commit 并 push
# 2026-06-10 Verification Update

- Shared contracts tests: `npm run test -w packages/shared-contracts` passed, 2 files / 8 tests.
- Desktop pet UI tests: `npm run test -w apps/desktop` passed, 1 file / 4 tests.
- Frontend build: `npm run build` passed for shared contracts and desktop Vite bundle.
- Rust tests: `cargo test --manifest-path src-tauri\Cargo.toml --locked --verbose` timed out after 300s with no compiler/test output and left idle `cargo` processes. Those processes were stopped. Rust unit verification remains pending because the local Cargo run does not progress.
- Code review fallback: internal review found a test isolation defect in `apps/desktop/src/features/pet/pet.test.tsx`; fixed by adding `afterEach(cleanup)`.
- Browser smoke test: Vite served `http://127.0.0.1:5173`; Playwright verified the pet button renders and right-click menu shows `打开历史 / 设置 / 暂停记录 / 退出`. Screenshot saved as `.playwright-mcp/prd01-pet-menu.png`.
- Manual Windows/Tauri verification still required for transparent borderless window, drag persistence after restart, history focus on double click, and right-click menu behavior in a real desktop shell.
