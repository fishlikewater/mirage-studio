# 前端目录结构规范

## 目标

- 页面、组件、状态和样式边界清晰
- 新人和 AI 能快速定位入口与依赖

## 推荐做法

- 画布领域代码优先放在 `src/features/canvas/` 下，按 `domain / application / ui / nodes / tools / models` 边界组织。
- 全局状态放在 `src/stores/`，但重业务逻辑应下沉到 `application` 或 `domain`。
- Tauri 命令封装放在 `src/commands/`，UI 不直接散落 `invoke` 调用。
- 通用 UI primitive 统一复用 `src/components/ui/primitives.tsx`。
- i18n 文案统一维护在 `src/i18n/locales/zh.json` 和 `src/i18n/locales/en.json`。
- 新节点相关文件至少同步检查 `domain/canvasNodes.ts`、`domain/nodeRegistry.ts`、`nodes/index.ts` 与具体节点组件。

## 不建议

- 页面文件承载大量业务逻辑
- 组件目录中混入请求、状态、样式、工具函数但没有边界
- 过早抽象通用组件
- 绕过 `nodeRegistry.ts` 在 UI 层手写节点类型白名单
