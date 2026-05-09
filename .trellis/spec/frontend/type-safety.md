# 前端类型安全规范

## 目标

- 前端、Tauri 命令、模型供应商协议和持久化数据契约同步
- 状态与组件 API 可推导

## 推荐做法

- 画布节点类型与数据结构优先从 `src/features/canvas/domain/canvasNodes.ts` 推导。
- 节点注册、默认数据和连线能力从 `nodeRegistry.ts` 推导，不在组件中复制类型判断。
- 模型与供应商能力从 `src/features/canvas/models/**` 和 Rust provider 注册同步维护。
- Tauri 命令 DTO 变更时，同时检查 `src/commands/**` 与 `src-tauri/src/commands/**`。
- 不手写重复的接口定义
- 变更接口时同步更新调用方、校验和展示层

## 检查项

- 是否存在类型与真实响应结构不一致
- 是否通过强转掩盖结构问题
- 是否缺少空值、错误态、边界态定义
- 是否新增 `previewImageUrl` 等图片字段但未进入去重编码/解码链路
- 是否新增 i18n key 但中英文语言包不一致
