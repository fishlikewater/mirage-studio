---
name: start
description: 开始一次 StoryBoard-Copilot 开发会话，读取项目上下文、规范与当前任务状态。
---

# 开始会话

用于新会话或长时间中断后恢复上下文。目标是先建立事实，再决定是否进入实现。

## 1. 读取核心规范

```bash
Get-Content AGENTS.md
Get-Content .trellis/workflow.md
Get-Content .trellis/spec/frontend/index.md
Get-Content .trellis/spec/backend/index.md
Get-Content .trellis/spec/guides/index.md
```

在类 Unix shell 中可用 `cat` 替代 `Get-Content`。

## 2. 获取当前上下文

```bash
python3 ./.trellis/scripts/get_context.py
python3 ./.trellis/scripts/task.py list
git status
```

检查是否已有当前任务、未提交改动、OpenSpec change 或正在进行的 session。

## 3. 判断任务级别

| 级别 | 判定 | 下一步 |
|------|------|--------|
| `L0` | 文档、测试、内部重构、工具配置，无用户可见行为变化 | 直接创建/选择 Trellis 任务 |
| `L1` | 单模块行为变化 | 先补 OpenSpec proposal/spec，再写计划 |
| `L2` | 跨层、持久化、Tauri 命令、模型协议或架构边界变化 | 补 proposal/spec/design，计划后执行 |

不确定时按更高一级处理。

## 4. 进入任务

```bash
python3 ./.trellis/scripts/task.py create "<title>" --slug <task-name>
python3 ./.trellis/scripts/task.py start <task-dir>
```

随后根据任务范围阅读具体 `.trellis/spec/` 文档，并把相关规范加入任务上下文。

## 5. 汇报格式

开始实现前，向用户简短说明：

- 当前任务理解
- 变更级别 `L0/L1/L2`
- 需要读取或更新的规范
- 初步验证命令

## 关键原则

- 先读规范，再动手。
- 行为变更先规格、再计划、再实现。
- 不要覆盖用户已有改动；遇到冲突先停下来说明。
- 修改 `SKILL.md` 时保持语言一致，并避免写成一次性任务清单。
