# Code Review 步骤

检查通过后运行独立 review；如果只能完成部分检查，也要在已知结果基础上运行 review，并记录缺口。Review 不是一次性动作，而是 loop：review -> 修复 -> 相关检查 -> 再 review，直到没有阻塞问题，或用户明确接受剩余风险。

## Reviewer 优先级

1. 项目定义的 review agent 流程。
2. 本地 Claude Code Agent，前提是已安装、已认证，并且适合当前仓库。
3. 可用的 Codex 子 agent。
4. Codex 内部 review fallback。

除非用户要求多个 reviewer，否则只使用一个主 reviewer。Reviewer 默认只读。

## Reviewer 上下文

提供：

- PRD 路径或摘要。
- design 文档路径。
- tasks 文档路径。
- 相对 base branch 的 diff。
- 已运行命令和结果。
- 已知验证缺口。
- PR readiness gate 状态。

不要把你自己的判断塞给 reviewer。要求 reviewer 关注 bug、回归、缺失测试、隐私/安全问题、契约不匹配。

## Diff 和范围

- 相对推断出的 base branch 计算 diff，例如 `git diff <base>...HEAD`，必要时包含未提交改动。
- 如果 diff 很大，提供文件列表、统计信息，并按模块提供重点 diff。
- 提供 design 文档和 tasks 记录，让 reviewer 能对照设计意图检查实现。
- 明确告诉 reviewer 不要修改文件，除非用户明确要求“review 并修复”。

## Claude Code Agent

只有在本地 Claude Code Agent 可用、已认证、且适合当前仓库时才使用。

检测建议：

- 先检查项目文档或 `AGENTS.md` 是否配置了 Claude 命令。
- 如果没有配置，在安全的前提下检查常见本地命令，例如 `claude --version`。
- 如果没有 CLI 或 agent 集成，不阻塞流程，直接 fallback。

运行方式：

- 尽量使用只读 review 模式。
- 传入 PRD/design/tasks 路径、diff 和检查结果。
- 把 Claude 输出保存或总结到 `tasks/<prd-slug>.md`。
- 除非用户明确要求 auto-fix reviewer，否则 review 阶段不允许 Claude 应用 patch。

建议 prompt：

```text
请作为独立 code reviewer 审查这次 PRD 驱动实现。

重点关注：
- 是否符合 PRD 和设计文档
- 行为回归
- 缺失测试
- 隐私/安全问题
- API/event/contract 不匹配
- 如适用，关注 Windows/Tauri/native 边界问题

请优先输出 findings，按严重程度排序，并尽量提供文件/行号。
如果没有阻塞问题，请明确说明，并列出剩余风险。
```

如果 Claude 不可用，使用可用子 agent。如果没有子 agent，执行内部 review，并标记为 fallback。

## 阻塞问题

- ready PR 前必须修复 P0/P1 findings，除非用户明确接受风险。
- 修复后重新运行相关测试/检查。
- 未解决 findings 必须写入 PR 风险。

## Review 修复 Loop

每一轮都记录到 `tasks/<prd-slug>.md`：

```markdown
| 轮次 | Reviewer | 结论 | 阻塞问题 | 修复结果 | 是否继续 review |
|---|---|---|---|---|---|
| 1 |  |  |  |  |  |
```

循环规则：

- 如果 reviewer 发现 P0/P1，或指出会导致验收失败的问题，必须修复。
- 修复后运行与改动相关的最小检查集。
- 检查通过后继续 review。
- 如果 reviewer 只剩 P2 或非阻塞建议，由 Codex 判断是否修复；不修复时记录风险。
- 如果用户明确接受某个阻塞风险，可以停止该项循环，但必须记录用户接受。
- 如果三轮后仍卡在同一问题，说明 blocker，并请求用户决策。

## 必需 Review 输出

记录：

- 使用的 reviewer。
- findings。
- 已应用修复。
- 剩余风险。
- review 是只读还是 patching。
- PR 可以 ready，还是应保持 draft。

把总结写入 `tasks/<prd-slug>.md` 和最终回复。
