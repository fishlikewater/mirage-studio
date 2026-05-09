---
name: check-cross-layer
description: 跨层检查，确认前端、状态、Tauri、持久化、模型协议与 i18n 已同步。
---

# 跨层检查

用于实现后或 review 前。多数回归不是技术不会，而是只改了一个层。

## 1. 识别改动范围

```bash
git status
git diff --name-only
```

按文件判断涉及哪些层：

- UI/节点：`src/features/canvas/nodes/**`、`src/features/canvas/ui/**`
- Domain/registry：`src/features/canvas/domain/**`
- Store：`src/stores/**`
- 应用服务/工具：`src/features/canvas/application/**`、`src/features/canvas/tools/**`
- 前端命令：`src/commands/**`
- Rust/Tauri：`src-tauri/src/**`
- i18n：`src/i18n/locales/**`

## 2. 必查维度

### 数据流

- 输入从哪里来，经过哪些层，最终写到哪里？
- 输出给谁消费，是否有下游依赖？
- 错误、空态、取消和失败是否逐层表达？

### 节点与工具

- 新节点是否同步 `canvasNodes.ts`、`nodeRegistry.ts`、`nodes/index.ts`？
- 菜单候选是否来自 registry，而不是 UI 白名单？
- 新工具是否同步类型、注册、编辑器和处理器？

### 模型与供应商

- 前端 provider/model registry 与 Rust provider 是否一致？
- API Key 设置页、别名、默认模型和请求路由是否同步？
- 缺失密钥、供应商失败、模型不支持参数时是否可读报错？

### 持久化

- SQLite schema 变更是否更新 `ensure_projects_table`？
- 新图片字段是否进入 imagePool 去重编码/解码？
- viewport 是否仍走轻量更新，不回退到整项目保存？

### i18n

- 新 key 是否同时存在于 `zh.json` 与 `en.json`？
- 组件中是否只使用 `t('stable.key')`？

## 3. 搜索建议

```bash
rg "节点类型或字段名" src src-tauri
rg "i18nKey" src/i18n/locales src
rg "command_name" src src-tauri
```

## 输出

汇报：

- 涉及的层
- 已确认同步的点
- 发现的问题与修复建议
- 尚未验证的风险
