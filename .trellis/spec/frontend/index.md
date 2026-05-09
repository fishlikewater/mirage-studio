# 前端开发规范

---

## 当前基线

- 前端形态：Tauri 桌面应用中的 React 单页画布应用。
- 技术栈：React 18、TypeScript、Zustand、@xyflow/react、TailwindCSS、i18next、Konva/react-konva。
- 核心入口：`src/App.tsx`、`src/features/canvas/Canvas.tsx`、`src/stores/projectStore.ts`、`src/stores/canvasStore.ts`。
- 契约来源：`src/features/canvas/domain/**`、`src/features/canvas/models/**`、`src/commands/**`、`src/i18n/locales/*.json`。

---

## 文档索引

| 文档 | 用途 |
|------|------|
| [目录结构](./directory-structure.md) | 说明模块划分和目录边界 |
| [组件规范](./component-guidelines.md) | 说明组件职责、组合与交互边界 |
| [Hook 规范](./hook-guidelines.md) | 说明复用逻辑与副作用边界 |
| [状态管理](./state-management.md) | 说明 Zustand、项目持久化和局部交互状态边界 |
| [质量规范](./quality-guidelines.md) | 说明前端门禁 |
| [类型安全](./type-safety.md) | 说明共享类型与契约同步原则 |
| [画布扩展](./canvas-extension-guidelines.md) | 说明节点、工具、模型和供应商扩展路径 |
| [性能规范](./performance-guidelines.md) | 说明画布交互、图片数据和持久化性能约束 |
| [i18n 规范](./i18n-guidelines.md) | 说明多语言 key、语言包和最低验证要求 |

---

**最低要求**：新增 UI 文案必须同步 `zh.json` 与 `en.json`，禁止在组件中硬编码中英文 fallback。
