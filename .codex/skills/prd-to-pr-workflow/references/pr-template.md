# PR 模板

创建或草拟 PR 时使用这个结构。

```markdown
## Summary

- 

## PRD / Design

- PRD:
- Base design:
- UI design:
- App design:
- Tasks:

## Behavior

- 

## Tests

- [ ] Unit tests
- [ ] Integration tests
- [ ] E2E tests
- [ ] Manual verification

Commands run:

```text

```

## Acceptance Criteria Mapping

| Criterion | Evidence | Status |
|---|---|---|
|  |  |  |

## Review

- Reviewer:
- Result:

## PR Readiness

- [ ] Design docs complete
- [ ] Design review loop complete
- [ ] TDD log complete or exceptions documented
- [ ] Checks pass or blockers documented
- [ ] Code review fix loop complete
- [ ] Blocking code review findings resolved or accepted
- [ ] E2E complete or gaps documented
- [ ] Human verification passed or explicitly waived
- [ ] Branch pushed
- PR type: Draft / Ready

## Risks / Follow-ups

- 
```

当检查、E2E、review 或人工验证不完整时，使用 Draft。只有所有 readiness gates 通过，或用户明确豁免 gate 时，才使用 Ready。
