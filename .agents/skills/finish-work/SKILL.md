---
name: finish-work
description: 收尾检查，在交付、提交或记录 session 前确认验证、文档与规范同步。
---

# 收尾检查

在声称“完成”之前使用。它不是形式化清单，而是防止遗漏验证、跨层同步和知识沉淀的最后一道门。

## 1. 按范围运行验证

前端类型检查：

```bash
npx tsc --noEmit
```

前端测试：

```bash
npm test
```

完整前端构建：

```bash
npm run build
```

Rust/Tauri 改动：

```bash
cd src-tauri && cargo check
```

如果某条命令未运行，必须说明原因。

## 2. 检查代码与规范同步

- 新增/修改节点：检查 `canvasNodes.ts`、`nodeRegistry.ts`、`nodes/index.ts`、菜单推导和连线能力。
- 新增/修改工具：检查 `tools/types.ts`、`builtInTools.ts`、tool editor、`toolProcessor.ts`。
- 新增/修改模型或供应商：检查前端 registry、Rust provider、API Key 设置链路和模型别名。
- 修改持久化：检查 `ensure_projects_table`、SQLite 字段、imagePool 编码/解码、viewport 轻量更新。
- 修改文案：检查 `zh.json` 与 `en.json` 均有同名 key。

## 3. 检查跨层影响

- UI 输入是否按 `UI -> Store -> 应用服务 -> Tauri 命令/API -> 持久化` 流动？
- 错误是否在每层都有可理解表达？
- 主路径、异常路径、空态、取消/失败场景是否覆盖？
- 是否引入拖拽、缩放、输入时的重计算或重写盘？

## 4. 检查文档与任务状态

- 行为变更是否已更新 OpenSpec？
- 新约束是否已沉淀到 `.trellis/spec/`？
- Trellis 任务状态、上下文和 session 是否需要更新？
- 发布相关改动是否已准备 `docs/releases/vx.y.z.md`？

## 输出

收尾时简短汇报：

- 已验证命令及结果
- 未运行命令及原因
- 仍然存在的风险或后续动作
