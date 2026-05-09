# 项目总览

## 项目目标与技术栈

- 产品目标：基于节点画布进行图片上传、AI 生成/编辑、工具处理（裁剪/标注/分镜）。
- 前端：React、TypeScript、Zustand、@xyflow/react、TailwindCSS、i18next、Konva/react-konva。
- 后端：Tauri 2、Rust 命令式接口、SQLite、rusqlite、WAL。
- 关键原则：解耦、可扩展、可回归验证、自动持久化、交互性能优先。

## 代码库浏览顺序

### 1. 入口与全局状态

- `src/App.tsx`
- `src/stores/projectStore.ts`
- `src/stores/canvasStore.ts`

### 2. 画布主流程

- `src/features/canvas/Canvas.tsx`
- `src/features/canvas/domain/canvasNodes.ts`
- `src/features/canvas/domain/nodeRegistry.ts`
- `src/features/canvas/NodeSelectionMenu.tsx`

### 3. 节点与覆盖层

- `src/features/canvas/nodes/*.tsx`
- `src/features/canvas/nodes/ImageEditNode.tsx`
- `src/features/canvas/nodes/GroupNode.tsx`
- `src/features/canvas/ui/SelectedNodeOverlay.tsx`
- `src/features/canvas/ui/NodeActionToolbar.tsx`
- `src/features/canvas/ui/NodeToolDialog.tsx`
- `src/features/canvas/ui/nodeControlStyles.ts`
- `src/features/canvas/ui/nodeToolbarConfig.ts`

### 4. 工具体系

- `src/features/canvas/tools/types.ts`
- `src/features/canvas/tools/builtInTools.ts`
- `src/features/canvas/ui/tool-editors/*`
- `src/features/canvas/application/toolProcessor.ts`

### 5. 模型与供应商适配

- `src/features/canvas/models/types.ts`
- `src/features/canvas/models/registry.ts`
- `src/features/canvas/models/image/*`
- `src/features/canvas/models/providers/*`
- `src-tauri/src/ai/providers/**`

### 6. Tauri 命令与持久化

- `src/commands/*.ts`
- `src/commands/projectState.ts`
- `src-tauri/src/commands/*.rs`
- `src-tauri/src/commands/project_state.rs`
- `src-tauri/src/lib.rs`

## 文档边界

- `.trellis/spec/` 定位为稳定技术规范，优先记录架构约束、分层规则、扩展流程、验证标准。
- 不记录易变的具体 UI 操作步骤、临时交互文案或产品走查细节；这些应放在需求文档、设计稿或任务说明中。
- 当实现变化仅影响交互细节而不影响技术约束时，可不更新本目录。
