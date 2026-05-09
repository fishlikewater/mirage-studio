# 前端组件规范

## 目标

- 组件职责单一
- 组合关系清晰
- 交互边界稳定

## 推荐做法

- 容器组件负责数据与状态，展示组件负责呈现
- 组件 API 保持简洁，避免“万能组件”
- 把复杂交互拆成可复用但边界明确的小组件
- 节点底部控制条优先复用 `src/features/canvas/ui/nodeControlStyles.ts`。
- 节点工具条位置与偏移统一复用 `src/features/canvas/ui/nodeToolbarConfig.ts`。
- 选中覆盖层 `SelectedNodeOverlay` 只承载轻量通用覆盖能力，节点核心输入区内聚在节点组件本体。
- 需要弹窗时复用项目已有对话框/primitive，并保留打开/关闭过渡。
- 组件文案通过 `useTranslation()` + 稳定 key 获取，不写字面量 fallback。

## 检查项

- 组件是否承担了过多业务逻辑
- Props 是否过多或命名不清
- 组件是否依赖过多外部状态
- 拖拽、缩放、输入态下是否会触发重计算或重持久化
- 中英文切换后关键按钮、错误提示是否可读且不截断
