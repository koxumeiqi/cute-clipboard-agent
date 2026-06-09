# 设计与任务产物

编码前必须先创建这些文件。

## 目录结构

```text
design/<prd-slug>/
  base-<prd-slug>.md
  ui-<prd-slug>.md
  app-<prd-slug>.md
tasks/
  <prd-slug>.md
```

如果项目已有自己的 design/tasks 目录约定，遵守项目约定，但保持这三个设计文档和一个任务文档的拆分。

## base-*.md

用途：对齐共享产品行为、公共契约、UI 与应用/原生层的集成边界。

必需章节：

```markdown
# <PRD 标题> - Base Design

## 来源
- PRD：
- 分支：
- 相关模块：

## 目标

## 非目标

## 共享领域模型

## 公共契约
### Commands / APIs
### Events
### Permissions
### Persistence

## UI-App 集成契约
### UI 调用
### 应用 / 原生响应
### 事件订阅
### 状态同步
### 错误映射
### 权限边界

## 集成边界

## BDD 场景
### 场景：...
Given ...
When ...
Then ...

## 成功标准
| 标准 | 来源 | 自动化证据 | 人工证据 |
|---|---|---|---|
|  |  |  |  |

## 风险

## 未决问题
```

## ui-*.md

用途：设计用户可见行为。

必需章节：

```markdown
# <PRD 标题> - UI Design

## 页面 / 窗口 / 入口

## 组件结构

## 状态
- loading
- ready
- empty
- error
- disabled / unauthorized

## 交互

## 视觉与可访问性规则

## UI BDD 场景

## E2E 验证计划
### 自动化范围
### 人工范围
### 环境要求
### 需要捕获的证据

## 人工验证清单
```

如果 PRD 没有直接 UI，写 `无直接 UI`，并描述用户可见效果、日志、CLI 行为或集成信号。

## app-*.md

用途：设计应用/原生/后端/领域行为。

必需章节：

```markdown
# <PRD 标题> - Application Design

## 运行时职责

## 模块变更

## 数据流

## 存储 / Schema

## 错误处理

## 隐私与安全

## CDD 契约
### 输入契约
### 输出契约
### 事件契约
### 权限契约
### 错误契约
### 持久化契约

## 测试策略
```

## tasks/<prd-slug>.md

用途：Codex 可执行的任务清单。

必需章节：

```markdown
# <PRD 标题> - Tasks

## 状态
- 分支：
- PRD：
- 设计：

## 计划
- [ ] 1. 根据 PRD/design 定义实现 slices
- [ ] 2. 运行设计产物 review loop
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
|  |  |  |  |  |  |

## 实现 Slices
| Slice | 行为 | 是否测试先行 | 负责区域 | 状态 |
|---|---|---|---|---|
|  |  |  |  |  |

## TDD 记录
| Slice | Red 命令 / 结果 | Green 命令 / 结果 | Refactor 说明 | Blocker / 替代验证 |
|---|---|---|---|---|
|  |  |  |  |  |

## 检查结果

## Review 结果
| 轮次 | Reviewer | 结论 | 阻塞问题 | 修复结果 | 是否继续 review |
|---|---|---|---|---|---|
|  |  |  |  |  |  |

## E2E 结果

## 人工验证

## PR 就绪度
- [ ] 设计文档完整
- [ ] 设计产物 review loop 完成，且无阻塞 PRD 偏离
- [ ] TDD 记录完整，或 blocker 已记录
- [ ] 检查完成，或 blocker 已记录
- [ ] 代码 review 修复 loop 完成
- [ ] 阻塞 review 问题已解决或被接受
- [ ] E2E 完成，或缺口已记录
- [ ] 人工验证已通过或被明确豁免
- [ ] 如需创建 PR，分支已 commit 并 push
```

推进过程中要持续更新任务状态，不要只在最后补。

对项目特定平台，需要加入平台门禁。对 Windows/Tauri 应用，相关 PRD 必须覆盖真实桌面窗口行为、剪贴板行为、托盘生命周期、SQLite 持久化、本地文件清理、隐私/权限边界。
