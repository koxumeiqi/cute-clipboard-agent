# 设计产物 Review

编码前必须 review 设计产物，目标是发现 PRD 偏离，而不是审查代码。

## Reviewer 上下文

提供：

- PRD 路径或 PRD 原文。
- `base-<prd-slug>.md`。
- `ui-<prd-slug>.md`。
- `app-<prd-slug>.md`。
- `tasks/<prd-slug>.md`。

不要提供“我觉得已经对齐”的结论。让 reviewer 独立判断。

## Review 重点

- 是否遗漏 PRD P0/P1 功能点。
- 是否加入 PRD 明确排除的非目标。
- `base-*.md` 是否定义 UI-App 集成契约。
- BDD 场景是否覆盖核心用户路径和异常路径。
- CDD 契约是否足够支撑实现。
- 成功标准是否可验证、可映射到 E2E 或人工验证。
- tasks 是否能一步一步执行，是否体现 TDD。
- 是否存在跨模块边界混乱、权限边界不清、隐私风险。

## 建议 Prompt

```text
请作为独立设计 reviewer，审查这些 workflow 产物是否准确覆盖 PRD，是否存在需求遗漏、越界设计、不可验证成功标准、BDD/CDD 不完整或任务计划不可执行。

只 review 文档，不修改文件。
请按严重程度输出 findings，并标明：
- PRD 对应来源
- 偏离或遗漏点
- 建议如何调整 base/ui/app/tasks 文档
如果没有阻塞偏离，请明确说明。
```

## 设计 Review Loop

每一轮都记录到 `tasks/<prd-slug>.md`：

```markdown
| 轮次 | Reviewer | 结论 | 偏离/阻塞问题 | 调整结果 | 是否继续 review |
|---|---|---|---|---|---|
| 1 |  |  |  |  |  |
```

循环规则：

- 如果存在 PRD P0/P1 遗漏、非目标越界、成功标准不可验证、核心契约缺失，必须调整文档。
- 调整后继续 review。
- 如果只剩非阻塞建议，可以记录为风险并继续进入编码。
- 如果用户明确接受某个偏离，可以停止该项循环，但必须记录用户接受。
- 如果三轮后仍卡在同一偏离，说明 blocker，并请求用户决策。

