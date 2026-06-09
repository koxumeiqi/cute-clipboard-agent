# PRD to PR Workflow Skill

项目级 skill 路径：

```text
.codex/skills/prd-to-pr-workflow
```

推荐触发方式：

```text
使用 $prd-to-pr-workflow 实现 docs/prd/04-剪贴板历史面板.md
```

或：

```text
按 PRD workflow 开发 PRD 08
```

## 这套流程会做什么

1. 读取项目上下文、PRD、架构和开发规范。
2. 检查工作区、base branch、remote、测试命令、PR 工具。
3. 创建或切换到 `codex/<prd-slug>` 分支。
4. 创建设计文档：
   - `design/<prd-slug>/base-<prd-slug>.md`
   - `design/<prd-slug>/ui-<prd-slug>.md`
   - `design/<prd-slug>/app-<prd-slug>.md`
5. 创建任务文档：
   - `tasks/<prd-slug>.md`
6. 启动只读子 agent 做设计产物 review；如偏离 PRD，调整后继续 review，直到无阻塞偏离或你明确接受风险。
7. 按 BDD/CDD 定义验收、接口、事件、权限、存储和错误行为。
8. 按 slice 执行 TDD：记录 Red / Green / Refactor，不能先测时记录 blocker 和替代验证。
9. 运行格式化、lint、类型检查、单元测试、集成测试、构建。
10. 启用只读 code review agent；如果 review 没过，修复后重新检查并继续 review。
11. 尽可能执行 E2E 验证，并记录自动化范围、人工范围和证据。
12. 完成人工验证 gate；未通过或未明确豁免时，不创建 ready PR。
13. 按 PR gate 创建 draft/ready PR，或准备 PR body。

## 可复用性

这套 skill 尽量不绑定本项目。复制到其他项目时，建议同步：

```text
.codex/skills/prd-to-pr-workflow/
```

然后在目标项目的 `AGENTS.md` 增加：

```markdown
PRD 驱动开发流程使用项目级 skill：`.codex/skills/prd-to-pr-workflow`。
当用户要求“按 PRD 开发”“实现某个 PRD”“执行 PRD workflow”“从需求到 PR”时，使用 `$prd-to-pr-workflow`。
```

## 本项目特定依赖

- PRD 默认在 `docs/prd/`。
- 设计文档默认写入 `design/<prd-slug>/`。
- 任务文档默认写入 `tasks/<prd-slug>.md`。
- 分支默认使用 `codex/` 前缀。
- 技术栈默认参考根目录 `AGENTS.md` 和 `docs/development-environment.md`。
- 本项目是 Windows/Tauri/React/TypeScript/Rust/SQLite 桌面应用，E2E 验证要优先覆盖桌宠窗口、历史面板、设置、隐私告知、托盘生命周期。

## Claude Code Agent

skill 已预留 Claude Code Agent review 环节，但不会强依赖它。review 默认只读，不允许 reviewer 自动改代码，除非你明确要求“review 并修复”。

使用优先级：

1. 项目明确配置的 reviewer。
2. 本地 Claude Code Agent。
3. 可用的 Codex sub-agent。
4. Codex 内部 review fallback。

如果后续要固定 Claude Code Agent 命令，可在 `AGENTS.md` 或 skill 的 `references/code-review.md` 中补充具体命令，例如可执行路径、参数格式、diff 输入方式。

## Ready PR 门禁

- 设计文档完整，并包含验收标准映射。
- 设计产物 review loop 已完成，且无阻塞 PRD 偏离。
- `tasks/<prd-slug>.md` 记录 TDD Red / Green / Refactor 或替代验证。
- 检查命令已执行，失败项有 blocker。
- 代码 review 修复 loop 已完成，阻塞问题已修复或被明确接受。
- E2E 已执行，或不可自动化部分已转入人工验证。
- 人工验证已通过，或你明确豁免。
- 分支已提交并推送。
