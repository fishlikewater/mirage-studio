# 画布扩展规范

## 节点注册单一真相源

- 节点类型、默认数据、菜单展示、连线能力统一在 `src/features/canvas/domain/nodeRegistry.ts` 声明。
- 不在 `Canvas.tsx`、`canvasStore.ts` 或 UI 组件中重复硬编码节点类型白名单。
- `connectivity` 是连线能力配置源：
  - `sourceHandle` / `targetHandle` 表示是否具备输入输出端口。
  - `connectMenu.fromSource` / `connectMenu.fromTarget` 表示从输出端或输入端拉线时是否允许出现在创建节点菜单。
- 菜单候选必须由注册表函数统一推导，例如 `getConnectMenuNodeTypes`。
- 内部衍生节点默认关闭 `connectMenu`，只能由工具或应用流程自动创建。

## 新模型接入

- 一模型一文件，放到 `src/features/canvas/models/image/<provider>/`。
- 模型定义必须声明：
  - `displayName`
  - `providerId`
  - 支持分辨率或比例
  - 默认参数
  - 请求映射函数 `resolveRequest`
- 涉及 Rust provider 时，同步检查 `src-tauri/src/ai/providers/**`、provider 注册、模型别名和 API Key 链路。

## 新工具接入

1. 在 `src/features/canvas/tools/types.ts` 声明能力。
2. 在 `src/features/canvas/tools/builtInTools.ts` 注册插件。
3. 在 `src/features/canvas/ui/tool-editors/` 新增对应编辑器。
4. 在 `src/features/canvas/application/toolProcessor.ts` 接入执行逻辑。
5. 产物必须走“处理后生成新节点”链路，不覆盖原节点。

## 新节点接入

1. 在 `domain/canvasNodes.ts` 增加类型与数据结构，必要时增加类型守卫。
2. 在 `domain/nodeRegistry.ts` 注册 `createDefaultData`、`capabilities`、`connectivity`。
3. 在 `nodes/index.ts` 注册渲染组件。
4. 明确手动创建策略：
   - 可手动创建：配置 `connectMenu.fromSource/fromTarget`。
   - 仅流程创建：关闭 `connectMenu`，由工具或应用服务触发。
5. 如新增分组/父子节点行为，必须同步验证删除、解组、连线清理与历史快照。
6. 节点内控制条优先复用 `nodeControlStyles.ts`，特化时基于统一 token 小幅覆盖。
7. 节点工具条必须复用 `nodeToolbarConfig.ts`，验证拖拽同步和多尺寸相对居中。
