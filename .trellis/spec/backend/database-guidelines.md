# 后端数据库规范

## 目标

- 模型、迁移、查询方式一致
- 数据变更可审计、可回滚、可复现

## 基本要求

- 当前 SQLite 表结构在 `ensure_projects_table` 中自愈；schema 变更必须同步 `PRAGMA table_info` 检查与 `ALTER TABLE` 逻辑。
- 查询优先使用项目已有 rusqlite 数据访问方式
- 事务边界由业务动作定义，不要为“方便”扩大事务
- WAL、项目快照、viewport 轻量更新与 history 持久化策略要保持一致
- 图片字段通过 `imagePool + __img_ref__` 去重编码；新增图片字段必须同步编码/解码映射
- 项目数据通过 `projectStore` 自动持久化，不要求手动保存。
- 重启默认进入项目页；进入项目时恢复上次 viewport。
- SQLite 库文件位于 Tauri `app_data_dir/projects.db`。
- `projects` 表核心字段包括 `nodes_json`、`edges_json`、`viewport_json`、`history_json`、`node_count`。
- 前端持久化采用双通道：
  - 整项目快照：`upsert_project_record`，防抖加 idle 调度。
  - 视口快照：`update_project_viewport_record`，轻量更新、独立防抖。

## 建议检查项

- 新表是否补齐主键、索引、审计字段
- 复杂查询是否可通过现有 rusqlite 封装表达
- 是否存在拼接 SQL / 条件遗漏 / N+1 查询
- 是否在错误层做了数据一致性补偿
- 是否破坏当前 `projects.db` 的基本可读性
- viewport 更新是否仍走 `update_project_viewport_record`

## 项目定制位

- 迁移方式：开发阶段以 `ensure_projects_table` 自愈为主。
- 查询框架：`rusqlite`。
- 关键字段：`nodes_json`、`edges_json`、`viewport_json`、`history_json`、`node_count`。
