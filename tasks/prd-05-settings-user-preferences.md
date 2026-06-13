# 设置与用户偏好 - 任务计划

## 状态
- 分支：`codex/prd-05-settings-user-preferences`
- PRD：`docs/prd/05-设置与用户偏好.md`
- 设计：`design/prd-05-settings-user-preferences/`

## 计划
- [x] 1. 根据 PRD/design 定义实现切片
- [x] 2. 运行设计产物 review loop
- [x] 3. 为 slice 1 创建失败测试
- [x] 4. 实现 slice 1 直到测试通过
- [x] 5. 对剩余 slices 重复 Red/Green/Refactor
- [x] 6. 串联集成边界
- [x] 7. 运行检查
- [x] 8. 运行代码 review 修复 loop
- [ ] 9. 运行 E2E 验证
- [ ] 10. 完成人工验证 gate
- [ ] 11. 准备或创建 PR

## 设计 Review 记录
| 轮次 | Reviewer | 结论 | 偏离/阻塞问题 | 调整结果 | 是否继续 review |
|---|---|---|---|---|---|
| 1 | 内部 fallback | 无阻塞。开机启动底层实现不属于 PRD-05 当前边界，但需在设计中说明。 | 无 | 已在非目标和风险中说明仅保存偏好。 | 否 |

## 实现切片
| 切片 | 行为 | 是否测试先行 | 负责区域 | 状态 |
|---|---|---|---|---|
| 1 | 共享契约和 Rust 聚合设置读写 | 是 | shared-contracts、src-tauri/settings、commands、events | 完成 |
| 2 | 设置窗口 UI 和 API | 是 | apps/desktop/features/settings、windows/settings | 完成 |
| 3 | 设置窗口路由和 Tauri 窗口接入 | 替代验证 | apps/desktop/settings.html、vite、src-tauri/windows | 完成 |

## TDD 记录
| 切片 | Red 命令 / 结果 | Green 命令 / 结果 | Refactor 说明 | 阻塞项 / 替代验证 |
|---|---|---|---|---|
| 共享契约 | `npm.cmd run test -w packages/shared-contracts -- settings`；契约新增后直接通过，用于锁定默认值、命令名和事件名。 | `npm.cmd run test -w packages/shared-contracts -- settings`：4 passed。 | 无。 | 无 |
| 设置窗口 UI | `npm.cmd run test -w apps/desktop -- settings-window`；失败，缺少 `./settings-window`。 | `npm.cmd run test -w apps/desktop -- settings-window`：4 passed。 | 表单保存逻辑集中在 `save`，失败回滚到上一次成功设置。 | 无 |
| Rust 聚合设置 | `cargo test --manifest-path src-tauri/Cargo.toml settings`；首次长编译超时。 | 延长超时后通过：8 passed。 | 聚合函数保持纯函数，command 负责事件发送。 | 首次超时为编译耗时，不是测试失败。 |

## 检查结果
- `npm.cmd run test -w packages/shared-contracts`：6 files / 24 tests passed。
- `npm.cmd run test -w apps/desktop`：3 files / 16 tests passed。
- `npm.cmd run build -w packages/shared-contracts`：通过。
- `npm.cmd run build -w apps/desktop`：通过。首次并行执行时 desktop build 早于 shared dist 更新而失败，顺序重跑已通过。
- `cargo test --manifest-path src-tauri/Cargo.toml`：29 passed。

## Review 结果
| 轮次 | Reviewer | 结论 | 阻塞问题 | 修复结果 | 是否继续 review |
|---|---|---|---|---|---|
| 1 | 内部 code review fallback | 无 P0/P1 阻塞。注意真实开机启动注册未接入，已按 PRD 边界作为偏好持久化处理。 | 无 | 无需修复。 | 否 |

## E2E 结果
- `Invoke-WebRequest http://127.0.0.1:1420/settings.html`：200，独立设置入口可访问。
- in-app Browser 自动化连接失败：`CreateProcessWithLogonW failed: 1326`，未能获取真实浏览器截图。
- 真实 Tauri 桌面交互待 E2E 验收 Agent 或人工验证。

## 人工验证
- 需要在 Windows Tauri 窗口中确认：从桌宠右键菜单打开设置页、切换设置后复制文本/图片的真实剪贴板行为、关闭待机动画后的桌宠动画变化、重启后设置保留。
- 开机启动当前仅保存偏好，真实系统注册由生命周期/打包模块后续接入。

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
