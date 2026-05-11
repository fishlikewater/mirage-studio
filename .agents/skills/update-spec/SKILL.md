---
name: update-spec
description: 将实现、调试或讨论中得到的新约束沉淀到 .trellis/spec。
---

# 更新 Code-Spec

当你发现新的实现规则、跨层契约、错误边界或踩坑经验时，使用本 skill 更新 `.trellis/spec/`。

## 何时更新

- 新增节点、工具、模型、供应商、Tauri 命令或持久化字段。
- 修复了一个非显而易见的 bug。
- 发现现有规范不足以指导后续实现。
- 行为变更导致前端、Rust、SQLite、i18n 或 OpenSpec 需要同步说明。
- 发现 `docs/superpowers/plans/*.md` 的状态回写规则不足，导致计划、OpenSpec 或 Trellis 状态不一致。

## 放在哪里

| 内容类型 | 位置 |
|----------|------|
| React 组件、Hook、状态、类型、i18n | `.trellis/spec/frontend/` |
| Tauri 命令、Rust provider、SQLite、错误、日志 | `.trellis/spec/backend/` |
| 编码前提醒、跨层思考、复用判断 | `.trellis/spec/guides/` |
| OpenSpec、Trellis、superpowers 计划状态同步 | `.trellis/spec/guides/spec-collaboration.md` |

## 必须写清楚

- 触发场景：什么时候必须遵守这条规则。
- 真实路径：相关文件或目录类型。
- 契约：命令名、字段名、状态流、错误表达。
- 验证：应该运行哪些命令或手测哪些路径。
- 反例：如果容易误用，写出“不应该这样做”。

## 更新流程

1. 读取目标 spec，避免重复。
2. 用简短、可执行的规则追加或改写。
3. 如果新增文档或章节，更新对应 `index.md`。
4. 运行必要的搜索，确认没有留下旧说法：

```bash
rg "旧术语|旧命令|旧路径" .trellis/spec
```

## 原则

- Code-Spec 写“如何安全实现”，不是写抽象口号。
- Guide 写“动手前要想什么”，不要复制实现细节。
- 不把一次性任务清单写进通用 skill 或 spec。
- 中文文档继续使用中文，避免中英混写。
