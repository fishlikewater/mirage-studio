# 跨层思考指引

## 什么时候必须看

- 变更穿过 3 层及以上
- Tauri 命令、状态、持久化、AI provider、异步链路发生变化
- 一个修改需要同步多个模块或多个进程

## 检查问题

- 输入从哪里来？经过哪些层？最终落到哪里？
- 输出给谁？哪些层依赖它？
- 错误在每一层如何表达和传播？
- API Key、provider 路由、模型别名和业务能力是否被混淆？
- 是否只改了主路径，漏掉异常路径和回滚路径？
- 是否需要同步 i18n、持久化编码/解码、历史快照或节点注册？

## 常见漏项

- `src-tauri/src/commands/**` 改了，但 `src/commands/**` 和调用方没改
- `nodeRegistry.ts` 改了，但 `canvasNodes.ts`、`nodes/index.ts` 或菜单推导没同步
- 新增模型/供应商只改前端或只改 Rust，导致运行时路由不到 provider
- SQLite 字段改了，但 `ensure_projects_table`、编码/解码、历史恢复没同步
