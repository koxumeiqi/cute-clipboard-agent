# TDD、BDD 与 CDD

## BDD

用 BDD 在实现前描述外部可观察行为。

格式：

```gherkin
场景：简短行为名称
Given 一个具体初始状态
When 用户或系统执行某个动作
Then 出现可观察结果
And 重要副作用也可被验证
```

好的 BDD 场景应该：

- 描述用户或系统可观察结果。
- 避免实现细节。
- 映射到 PRD 验收标准。
- 尽可能转化为 E2E 或集成测试。

## CDD

用 CDD 在模块串联前定义契约。

需要定义：

- command/API 名称。
- 输入 DTO。
- 输出 DTO。
- 事件和 payload 结构。
- 权限等级。
- 存储 schema 和 migration 行为。
- 错误码/错误信息。
- 隐私约束。

事件驱动系统必须定义事件允许携带完整内容、预览内容、元信息，还是只允许携带 ID。

## TDD

先写测试，再写代码。例外情况要作为 blocker 记录，不能静默跳过。

推荐顺序：

1. 契约测试：DTO、事件、commands、权限默认值。
2. 领域测试：纯逻辑。
3. Repository/Storage 测试：持久化。
4. 集成测试：跨模块行为。
5. UI 测试：用户交互。
6. E2E 测试：成功标准。

每个实现 slice 都要记录：

- Red 命令和预期失败。
- Green 命令和通过结果。
- Refactor 说明。
- Red 无法运行时的 blocker 和替代验证。

如果因为项目未初始化、工具缺失、行为只能人工验证，或风险不足以新增测试而无法先写失败测试，必须先写明 blocker 和替代验证，再开始实现。

不要把 slice 标记完成，除非满足以下任一条件：

- 已记录 Red/Green/Refactor。
- 已记录 blocker 和替代验证。
- 用户明确接受该 slice 不写测试。

## 测试命名

测试名描述行为：

- `publishes_metadata_only_clipboard_created_event`
- `trims_history_to_configured_capacity`
- `does_not_record_when_recording_is_paused`
- `opens_history_panel_from_pet_double_click`

