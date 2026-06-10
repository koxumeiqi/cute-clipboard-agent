---
name: prd-to-pr-workflow
description: PRD 驱动的软件开发 workflow skill，用于规范从需求理解、设计拆解、任务规划、分支管理、TDD 编码、检查、代码审查、E2E 验证、人工验收到 PR 准备/创建的完整流程。当用户要求根据 PRD 开发功能、实现某个 PRD 单元、执行 PRD workflow、制定可复用 AI coding 流程、或按 BDD/CDD/TDD 从需求推进到 PR 时触发。
---

# PRD 到 PR 开发流程

使用本 skill 执行可复用的 PRD 驱动编码流程。流程尽量保持跨项目可迁移；只有当项目中存在特定上下文时，才读取并遵守这些上下文。

## 必要输入

- PRD 来源：文件路径、粘贴文本、issue 链接或用户描述。
- 目标范围：PRD ID、PRD 名称或功能名称。
- PR 目标：只有在无法推断远程仓库或 base branch 时才询问用户。

## 核心规则

不要直接开始实现。固定顺序是：读取 PRD 和项目上下文，检查工作区，创建或切换功能分支，创建设计和任务文档，运行设计产物 review loop，定义 BDD/CDD/TDD 门禁，先写失败测试，再实现，检查，运行代码 review 修复 loop，验证，在需要时等待人工验证，然后准备或创建 PR。

## 工作流程

1. 读取项目上下文并检查工作区。
   - 遵守当前会话已加载的项目上下文，例如根目录 `AGENTS.md`。
   - 只有当项目上下文明显缺失、与用户要求冲突，或用户要求核对时，才主动读取 `AGENTS.md`。
   - 读取 PRD 来源。
   - 如果存在相关架构/开发文档，读取必要内容，例如 `docs/architecture/`、`docs/dev/` 或项目自定义规则。
   - 检查当前 `git status`，避免覆盖用户或其他 agent 的无关改动。
   - 尽量推断 base branch、remote、测试命令、构建命令和 PR 工具。
   - 如果工作区有无关脏改动，必须保留它们。只有在分支操作和后续编辑不会覆盖这些改动时才继续。

2. 创建功能分支。
   - 如果项目上下文定义了分支前缀，使用项目约定；否则默认使用 `codex/`。
   - 分支名由 PRD ID 和短 slug 组成，例如 `codex/prd-04-history-panel`。
   - 如果分支已存在，先检查分支状态再切换，避免破坏已有工作。

3. 创建 workflow 产物。
   - 创建 `design/<prd-slug>/`。
   - 创建 `design/<prd-slug>/base-<prd-slug>.md`，用于共享契约、接口、集成边界、成功标准和 BDD 场景。
   - 创建 `design/<prd-slug>/ui-<prd-slug>.md`，用于 UI/窗口/组件设计、状态、交互、可访问性和 UI BDD。
   - 创建 `design/<prd-slug>/app-<prd-slug>.md`，用于后端/原生/领域设计、存储、服务、权限、事件和 CDD。
   - 创建 `tasks/<prd-slug>.md`，用于一步一步执行的任务清单。
   - 所有生成文档必须以中文为主；除 PRD、UI、API、DTO、BDD、CDD、TDD、E2E、PR、Red/Green/Refactor 等必要工程术语外，不要生成英文标题或英文段落。
   - 创建文档前读取 `references/design-artifacts.md`。

4. 运行设计产物 review loop。
   - 启动一个只读子 agent 审查 `base-*.md`、`ui-*.md`、`app-*.md`、`tasks/<prd-slug>.md` 是否偏离 PRD。
   - 给 reviewer 提供 PRD 来源、设计文档路径和任务文档路径，不要提供你的自我判断。
   - Reviewer 必须重点检查：PRD 需求遗漏、非目标越界、BDD/CDD 不完整、UI-App 集成契约缺失、成功标准不可验证、任务计划不可执行。
   - 如果 reviewer 指出偏离 PRD 或阻塞问题，立即调整设计/任务文档。
   - 调整后继续 review，直到没有阻塞偏离，或用户明确接受剩余风险。
   - 把每轮 review 结论、调整内容和剩余风险记录到 `tasks/<prd-slug>.md`。

5. 编码前定义验收。
   - 把 PRD 成功标准转换为可验证的验收标准。
   - 用 Given/When/Then 写 BDD 场景。
   - 定义 CDD 契约：commands、events、DTO、权限、持久化 schema、错误行为。
   - 定义 TDD 计划：实现前要先写哪些测试。
   - 定义 E2E 和人工验证门禁：哪些可以自动化、哪些必须真实环境验证、成功证据是什么。

6. TDD 先行实现。
   - 针对最小可用 slice，先写或更新失败测试。
   - 运行目标测试，并把失败命令和预期失败原因记录到 `tasks/<prd-slug>.md`。
   - 写最少实现让测试通过。
   - 运行同一组目标测试，并记录通过结果。
   - 只有行为变绿后再重构。
   - 按任务清单重复推进。
   - 如果无法先写或运行失败测试，必须先记录 blocker 和替代验证方式，再实现。
   - 除非用户要求，否则 commit 可选。

7. 运行检查。
   - 运行项目可用的 format、lint、typecheck、单元测试、集成测试和构建命令。
   - 如果命令未知，检查 package manifest、Cargo 文件、Makefile、任务脚本、CI 配置或文档。
   - 在最终回复和任务文档中记录命令和结果。

8. 运行代码 review 修复 loop。
   - 使用 `Review Agent Selection` 中的统一优先级。
   - reviewer 默认只读；除非用户明确要求修复，否则 reviewer 不得改文件。
   - 给 reviewer 提供 PRD/design/task 路径、相对 base branch 的 diff、检查结果。
   - 如果 reviewer 指出 P0/P1 或会导致验收失败的问题，先修复，再重新运行相关检查，再继续 review。
   - Review -> 修复 -> 检查 -> 再 review 是循环，直到没有阻塞问题，或用户明确接受剩余风险。
   - 如果没有可用子 agent 或 Claude 集成，执行内部 code review，并明确标记为 fallback；fallback 也必须按同样的修复循环处理。
   - 调用或模拟 review 前读取 `references/code-review.md`。

9. 验证功能行为。
   - 在工具和环境允许时，运行 `ui-*.md` 和 `base-*.md` 中的 E2E 计划。
   - 对 Web/桌面 UI，优先使用浏览器或应用自动化工具。
   - 对照 `base-*.md` 的成功标准验证结果。
   - 如果完整 E2E 不现实，必须记录原因、最接近的自动化验证方式，以及剩余人工检查项。

10. 人工验证 gate。
   - 在 `tasks/<prd-slug>.md` 或最终回复中补充简洁的人工验证清单。
   - 写清具体场景、预期结果和环境假设。
   - 人工验证未通过，且用户没有明确授权跳过时，不创建 ready PR。
   - 如果用户希望先继续自动化流程，只创建 draft PR 或提供 PR body，并标记人工验证待完成。

11. 准备 PR。
   - 总结设计文档、实现、测试、review 结果和已知风险。
   - 检查 base branch、head branch、remote、commit 状态、push 状态、PR CLI/工具和认证。
   - 只有检查通过、阻塞 review 已解决、人工验证 gate 已满足或被明确豁免时，才创建 ready PR。
   - 如果检查不完整、review 只是 fallback、或人工验证待完成，优先创建 draft PR。
   - 如果无法创建 PR，提供分支名和 PR 描述正文。

## 产物命名

- 从 PRD ID 和标题推导 `<prd-slug>`。
- 使用小写 kebab-case。
- 示例：
  - `prd-04-history-panel`
  - `prd-08-event-bus-agent-bridge`
  - `settings-user-preferences`

## 分支命名

- 默认：`codex/<prd-slug>`。
- 如果项目上下文定义了分支前缀，使用项目约定。
- 分支名应稳定且简短。

## Review Agent 选择顺序

1. 项目定义的 review agent 流程。
2. 本地 Claude Code Agent，前提是已安装、已认证，并且适合当前仓库。
3. 可用的 Codex 子 agent。
4. 内部 code review fallback。

不要因为首选 reviewer 不可用而阻塞实现。必须记录 fallback。

## PR 门禁

创建 ready PR 前必须满足：

- 设计产物已存在，并包含成功标准。
- 设计产物 review loop 已完成，没有阻塞 PRD 偏离，或用户明确接受剩余风险。
- 任务清单已记录 TDD Red/Green/Refactor，或记录了替代验证。
- 必要检查已运行，或 blocker 已记录。
- 代码 review 修复 loop 已完成，阻塞问题已修复，或用户明确接受风险。
- E2E 验证已运行，或不可自动化的缺口已记录。
- 人工验证已通过，或用户明确豁免。
- 创建托管 PR 时，分支已 commit 并 push。

## 引用文件

- 创建设计和任务文档前，读取 `references/design-artifacts.md`。
- 运行设计产物 review loop 前，读取 `references/design-review.md`。
- 写测试或验收标准前，读取 `references/tdd-bdd-cdd.md`。
- 调用或模拟 review 前，读取 `references/code-review.md`。
- 准备 PR 正文前，读取 `references/pr-template.md`。
