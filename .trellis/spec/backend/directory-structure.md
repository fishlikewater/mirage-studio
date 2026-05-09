# 后端目录结构规范

## 目标

- 模块职责清晰
- 包结构稳定
- 新人和 AI 能快速定位代码

## 推荐做法

- Tauri 命令按能力放在 `src-tauri/src/commands/`，前端命令封装放在 `src/commands/`。
- AI 供应商适配按 provider 拆在 `src-tauri/src/ai/providers/`，一类供应商一个清晰边界。
- 前端模型定义放在 `src/features/canvas/models/`，Rust provider 与前端 registry 必须同步。
- 持久化相关逻辑集中在 `project_state.rs` 等命令/存储模块，不在多个命令中复制 SQL。
- 命令层只做参数校验、错误转换和响应编排；业务规则与数据访问保持可定位边界。

## 命名建议

- Rust 模块名使用 snake_case，前端 TypeScript 文件遵循现有 camelCase/PascalCase 风格。
- Tauri 命令名保持稳定、语义化，避免随 UI 文案变动。
- Provider/model id 使用稳定小写命名，并与前端 `providerId`、模型 id 保持一致。

## 不建议

- 一个模块同时承担多个无关业务
- 在 Tauri 命令函数中堆大量业务逻辑或 SQL 拼接
- 在公共模块中混入业务特化逻辑
- 前端新增 provider 后忘记注册 Rust provider，或反向漏同步
