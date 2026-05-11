# OpenSpec、superpowers 与 Trellis 协作规范

## 职责划分

- `OpenSpec` 负责正式行为变更工件：`proposal.md`、`design.md`、`specs/**/*.md`、`tasks.md`。
- `superpowers` 负责以正式工件为输入开展探索、方案确认、计划编写和执行方法选择。
- `Trellis` 负责执行过程、上下文注入、任务目录、journal 留痕和多 Agent 协作脚本。

三者不是并行体系，而是：`OpenSpec` 管正式规格，`superpowers` 管思考与计划，`Trellis` 管执行上下文和记录。

## 变更分级

- `L0`：纯文档、测试、重构、工具配置或无用户可见行为变化，可直接进入 Trellis。
- `L1`：单模块或单能力行为变化，先补 OpenSpec proposal/spec，再写计划。
- `L2`：跨模块、跨层、接口契约、持久化结构、模型供应商协议或架构边界变化，必须补 OpenSpec proposal/spec/design。

## 同步规则

- `superpowers:brainstorming` 与 `superpowers:writing-plans` 必须以对应 OpenSpec 工件为输入，不得另起一套范围或计划。
- 开发计划进入执行前，必须检查并对齐 `openspec/changes/<change>/tasks.md`。
- `docs/superpowers/plans/*.md` 是执行期活文档，不是一次性附件；进入执行后必须持续回写状态。
- 若 OpenSpec `tasks.md` 有增删、拆分、合并、重排或状态变更，开发计划必须同步调整。
- 若开发计划有实质性变更，必须同时检查并更新 OpenSpec `tasks.md`。
- 对 `L1/L2`，没有 active OpenSpec change 时，不进入正式编码实现。

## 开发计划活文档规则

- 计划中的任务和步骤必须使用 checkbox 语法，便于在执行中从 `- [ ]` 更新为 `- [x]`。
- 计划必须包含“当前执行状态”或同等区块，用于记录日期、执行批次、阻塞点、验证结果和 review 结论。
- 只有“已实现且验证通过”的步骤才能勾选；被阻塞、未验证或只完成部分实现的步骤必须保持未勾选，并说明原因。
- 每轮实现、验证、review 或范围变更后，都要同步更新计划文档。
- 收尾前必须核对计划状态、Trellis 任务状态和 OpenSpec `tasks.md` 不冲突。

## 验收规则

- `L1/L2` 完成后必须运行 OpenSpec 校验：

```bash
openspec validate --strict --type change <slug>
```

- 归档前必须确认实现、测试、计划状态和 OpenSpec 任务状态一致。
