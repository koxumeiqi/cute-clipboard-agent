# 剪贴板历史栈与本地存储 - 任务计划

## 状态
- 分支：当前 `codex/prd-02-clipboard-listener-normalization`，因工作区存在前序未提交改动，本单元未切新分支。
- PRD：`docs/prd/03-剪贴板历史栈与本地存储.md`
- 设计：`design/prd-03-clipboard-history-storage/`

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
| 1 | 内部 fallback | 通过 | 设计覆盖 STORE-01 到 STORE-09；图片字节保存仍依赖后续图片模块增强，不阻塞本单元历史生命周期 | 在基础设计风险中记录 `pending://` 上游路径限制 | 否 |

## 实现切片
| 切片 | 行为 | 是否测试先行 | 负责区域 | 状态 |
|---|---|---|---|---|
| 1 | 历史栈入栈、倒序查询、默认容量 20 和容量裁剪 | 是 | `src-tauri/src/history` | 已完成 |
| 2 | SQLite schema、启动加载、持久化开关 | 是 | `src-tauri/src/storage`、`history` | 已完成 |
| 3 | 删除、清空、图片文件清理 | 是 | `history`、`image` | 已完成 |
| 4 | Tauri commands、事件和共享契约 | 是 | `commands`、`events`、`packages/shared-contracts` | 已完成 |
| 5 | 剪贴板监听集成历史入栈 | 是 | `clipboard`、`lib` | 已完成 |

## TDD 记录
| 切片 | Red 命令 / 结果 | Green 命令 / 结果 | Refactor 说明 | 阻塞项 / 替代验证 |
|---|---|---|---|---|
| 1 | `cargo test --manifest-path src-tauri/Cargo.toml history::tests` 编译失败：新增 setup 使用 `state` 未导入 `tauri::Manager` | 修复导入后同命令通过，9 个历史测试通过 | 无 | 无 |
| 2 | `cargo test --manifest-path src-tauri/Cargo.toml history::tests` 初次覆盖新增持久化测试 | 同命令通过，验证 SQLite 恢复和持久化关闭不恢复 | 无 | 无 |
| 3 | `cargo test --manifest-path src-tauri/Cargo.toml history::tests` 覆盖图片删除测试 | 同命令通过，删除图片历史后原图和缩略图不存在 | 无 | 无 |
| 4 | `npm run test -w packages/shared-contracts` 覆盖事件和 command 名称 | 命令通过，4 个文件 16 个测试通过 | 无 | 无 |
| 5 | `cargo test --manifest-path src-tauri/Cargo.toml` 覆盖调试入口集成编译 | 命令通过，监听和 debug 入口均先入历史再发事件 | 无 | 无 |

## 检查结果

- `cargo test --manifest-path src-tauri/Cargo.toml`：通过，24 个 Rust 测试通过。
- `npm run test -w packages/shared-contracts`：通过，4 个测试文件 16 个测试通过。
- `npm run test -w apps/desktop`：通过，1 个测试文件 6 个测试通过。
- `cargo check --manifest-path src-tauri/Cargo.toml`：通过。
- `npm run build`：通过，共享契约和桌面前端完成生产构建。

## Review 结果
| 轮次 | Reviewer | 结论 | 阻塞问题 | 修复结果 | 是否继续 review |
|---|---|---|---|---|---|
| 1 | 内部 fallback | 通过 | 无 P0/P1 阻塞问题；事件未泄露完整文本，删除/清空会清理图片文件，SQLite 持久化开关有测试覆盖 | 无需修复 | 否 |

## E2E 结果

当前 PRD 无直接 UI，未启动浏览器/Tauri 窗口。以 Rust 单元测试、共享契约测试、前端测试、构建和 `cargo check` 作为自动化验证。剩余真实桌面验证项记录在人工验证清单中。

## 人工验证

未执行真实 Windows 剪贴板/Tauri 窗口人工验证。需要用户或后续验收在本机执行：

- 复制文本后调用历史列表，确认新条目位于顶部。
- 连续复制超过 20 条，确认旧记录被裁剪。
- 调整容量为 10，确认历史数量不超过 10。
- 删除单条历史，确认查询不到该项。
- 清空历史，确认列表为空；若包含图片历史，确认原图和缩略图文件被删除。
- 关闭持久化后复制新内容，重启应用确认关闭期间的新历史不恢复。

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
