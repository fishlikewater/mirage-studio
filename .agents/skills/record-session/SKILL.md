---
name: record-session
description: 在任务完成、验证结束或文档会话收尾后记录 Trellis session。
---

# 记录 Session

用于把本次工作沉淀到 `.trellis/workspace/`。业务代码提交由当前会话约定决定；本 skill 只负责记录会话与 Trellis 元数据。

## 1. 读取记录上下文

```bash
python3 ./.trellis/scripts/get_context.py --mode record
python3 ./.trellis/scripts/task.py list
git status
git log --oneline -10
```

确认当前任务、最近提交和工作区状态。

## 2. 归档已完成任务

只有当验收标准实际满足时才归档：

```bash
python3 ./.trellis/scripts/task.py archive <task-name>
```

不要只因为 `task.json` 状态看起来完成就归档；以真实验收与验证为准。

## 3. 写入 session

```bash
python3 ./.trellis/scripts/add_session.py \
  --title "Session Title" \
  --commit "hash1,hash2" \
  --summary "Brief summary"
```

记录内容应包含：

- 本次目标
- 主要变更
- 验证命令与结果
- 未完成风险或后续动作

## 4. 结束检查

- `.trellis/workspace` 已更新。
- `.trellis/tasks` 状态与真实任务一致。
- 没有把业务代码和 session 元数据混淆成同一个“已验证”结论。
