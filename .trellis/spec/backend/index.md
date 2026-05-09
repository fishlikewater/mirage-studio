# 后端开发规范

---

## 当前基线

- 架构形态：Tauri 2 命令式后端，作为桌面应用的本地能力与 AI/provider 适配层。
- 运行模块：`src-tauri/src/lib.rs`、`src-tauri/src/commands/**`、`src-tauri/src/ai/providers/**`。
- 技术栈：Rust、Tauri 2、rusqlite、SQLite WAL、serde。
- 规范来源：以仓库内已落地代码、配置、测试和迁移脚本为准

---

## 阅读顺序

| 文档 | 用途 |
|------|------|
| [目录结构](./directory-structure.md) | 模块划分、包结构、命名方式 |
| [数据库规范](./database-guidelines.md) | 实体、迁移、查询、事务 |
| [异常处理](./error-handling.md) | 错误码、异常边界、统一返回 |
| [日志规范](./logging-guidelines.md) | 结构化日志、敏感信息、链路标识 |
| [质量规范](./quality-guidelines.md) | 测试、lint、门禁、禁止模式 |

---

## 使用原则

- 优先遵循仓库现有模式，不新增与现有风格冲突的写法
- 先查已有公共模块、公共库或基础设施封装
- 涉及跨模块、异步链路、权限边界时，同时阅读 `guides/` 下的思考指引

---

**文档语言**：默认中文，可按项目统一改为英文。
