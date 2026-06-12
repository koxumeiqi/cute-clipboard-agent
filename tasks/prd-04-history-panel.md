# 剪贴板历史面板 - 任务计划

## 状态
- 分支：`codex/prd-04-history-panel`
- PRD：`docs/prd/04-剪贴板历史面板.md`
- 设计：`design/prd-04-history-panel/`

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
| 1 | 内部 review fallback | 通过 | 无阻塞。重点确认 PRD 非目标未引入搜索/筛选/Agent 操作；图片恢复作为 PRD P0 保留。 | 无需调整 | 否 |

## 实现切片
| 切片 | 行为 | 是否测试先行 | 负责区域 | 状态 |
|---|---|---|---|---|
| 1 | 历史面板展示中文文案、文本/emoji 预览、图片缩略图、空态/错误态 | 是 | `features/history-panel` | 完成 |
| 2 | 恢复、删除、清空交互调用正确 API 并更新 UI | 是 | `features/history-panel`、`shared/api` | 完成 |
| 3 | Rust 恢复命令支持图片历史写回并保持写回抑制 | 是 | `src-tauri/src/commands`、`src-tauri/src/clipboard` | 完成 |
| 4 | 构建、测试、人工验证记录 | 否，验证型任务 | 全局 | 待开始 |

## TDD 记录
| 切片 | Red 命令 / 结果 | Green 命令 / 结果 | Refactor 说明 | 阻塞项 / 替代验证 |
|---|---|---|---|---|
| 1 | `npm run test -w apps/desktop -- history-window.test.tsx`，5 个测试失败：图片无缩略图、图片项不可恢复、删除按钮无条目化标签、空态文案不匹配。 | 同命令通过：5 个测试通过。 | 调整 history 面板可访问标签、图片预览固定结构、恢复成功文案。 | 无 |
| 2 | 同 slice 1 Red 覆盖恢复/删除/清空交互失败。 | 同命令通过：恢复、删除、清空交互通过。 | 增加单条 busy 状态，删除按钮独立 aria-label。 | 无 |
| 3 | `cargo test --manifest-path src-tauri/Cargo.toml image_data_from_path` 编译失败：缺少图片解码依赖。 | 添加 `image` crate 后同命令通过：2 个测试通过。 | 将 restore command 改为 `write_item_to_clipboard`，按文本/图片分派。 | Windows 真实图片剪贴板写回仍需人工验证 |

## 检查结果
- `npm test`：通过。shared-contracts 16 个测试通过，desktop 11 个测试通过，Rust 26 个测试通过。
- `npm run build`：通过。shared-contracts TypeScript 编译通过，desktop Vite 生产构建通过。

## Review 结果
| 轮次 | Reviewer | 结论 | 阻塞问题 | 修复结果 | 是否继续 review |
|---|---|---|---|---|---|
| 1 | 内部 code review fallback | 通过 | 无 P0/P1 阻塞。剩余风险是 Browser 插件受 Windows 沙箱启动问题影响，未能完成页面自动化截图；真实图片剪贴板写回仍需 Windows 手工验证。 | 已记录 E2E 和人工验证缺口 | 否 |

## E2E 结果
- 已尝试启动 Vite dev server 并使用 Browser 插件打开 history 页面。
- Browser 连接连续失败，错误来自本机 Windows 沙箱启动层：`windows sandbox failed: spawn setup refresh`。
- 替代验证：前端组件测试覆盖 history 页面核心交互，`npm run build` 确认 history 入口可编译打包。
- 未完成项：真实 Tauri 窗口中的双击打开、文本/emoji/图片系统剪贴板写回需要人工验证。

## 人工验证
待 Windows Tauri 真实窗口验证：
- 双击桌宠打开历史面板。
- 复制文本、emoji、图片后确认列表预览。
- 点击文本/图片项后确认系统剪贴板内容恢复。
- 删除单条后确认列表更新。
- 清空后确认空状态。

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
