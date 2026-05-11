# superpowers 文档目录

本目录用于保存 `superpowers` 工作流产生的设计稿和实现计划。

## 目录职责

- `specs/`：方案讨论后的设计文档，通常由 `superpowers:brainstorming` 产出。
- `plans/`：可执行开发计划，通常由 `superpowers:writing-plans` 产出。

## 计划活文档规则

- `plans/*.md` 进入执行阶段后必须作为活文档维护。
- 任务步骤使用 checkbox 语法：未完成为 `- [ ]`，完成且验证通过后改为 `- [x]`。
- 计划中应包含“当前执行状态”或同等区块，用于记录日期、执行批次、阻塞、验证和 review 结论。
- 未完成、未验证或被阻塞的步骤不得误勾选；应保留未完成状态并说明原因。
- 收尾前必须核对计划状态、Trellis 任务状态和 OpenSpec `tasks.md` 不冲突。

## 与其他流程的关系

- 行为变更规格以 `openspec/` 为准。
- 执行上下文和 session 留痕以 `.trellis/` 为准。
- 计划文档负责承接执行步骤与状态回写，不替代 OpenSpec 或 Trellis。
