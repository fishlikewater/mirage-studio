# OpenAI Image Protocol Design

## 分级

本变更为 L2：

- 前端设置页增加接入协议选项。
- 前端运行时模型配置需要向 Tauri 传递新的协议和动作字段。
- 后端 Tauri 命令需要根据协议和动作分流到 OpenAI Images API 的生成或编辑接口。
- 图像节点 UI 行为随供应商协议变化。

## 方案选择

采用“显式生成 / 编辑动作”的方案：

- `生成`按钮保持现有语义，调用 OpenAI `images/generations`。
- `编辑`按钮只在当前模型来自 `openai-image` 协议时显示，调用 OpenAI `images/edits`。
- `编辑`按钮复用当前节点通过 `@` 关联解析出的参考图片列表。

放弃“有参考图就自动编辑”的方案，因为它会让同一个“生成”按钮根据上下文隐式切换接口，用户不容易判断实际调用行为，也容易破坏现有供应商的生成体验。

放弃新增独立编辑节点或独立选图器，因为本项目已经有 `@` 关联节点图片作为参考图的逻辑；复用该链路能减少 UI 与数据流分叉。

## 前端设计

### 配置模型

`CustomProviderProtocol` 增加 `openai-image`。它复用 OpenAPI 连接字段：

- `connection.openapi.baseUrl`
- `connection.openapi.apiKey`

这样可以避免新增第三套连接字段，也与 OpenAI Images API 的 “Base URL + Bearer API Key + model” 模型保持一致。

### 运行时模型

`RuntimeProviderConfig.protocol` 允许 `openai-image`。运行时模型仍由自定义供应商模型列表生成，`remoteModelId` 继续来自模型项配置。

### 动作字段

`GenerateImagePayload` / Tauri DTO 增加可选动作字段：

- `action?: "generate" | "edit"`

默认值为 `generate`，保证现有调用方不需要全部修改。只有新增的编辑按钮传 `edit`。

### 参考图来源

编辑按钮不新建选图逻辑。它依赖 `ImageEditNode` 已经计算出的 `incomingImages`：

- `incomingImages` 由现有图谱引用和 `@` 关联解析逻辑产生。
- `openai-image` 在 `tauriAiGateway.normalizeReferenceImages` 中应与普通自定义供应商一致，把本地图片转换为 data URL。
- 后端直接把 data URL 填入 JSON `images[].image_url`。

### UI

在 `ImageEditNode` 底部操作区：

- 非 `openai-image`：维持单个“生成”按钮。
- `openai-image`：显示“生成”和“编辑”两个按钮。
- “编辑”按钮在无参考图、正在生成或供应商未配置时禁用。

## 后端设计

新增 `src-tauri/src/ai/providers/openai_image/` 或单文件模块，职责只处理 OpenAI Images API 兼容协议：

- 校验 `baseUrl`、`apiKey`、`remoteModelId`。
- `generate_image` 构造 `/images/generations` JSON 请求。
- `edit_image` 构造 `/images/edits` JSON 请求。
- 统一解析响应：
  - 优先 `data[0].b64_json` -> `data:image/png;base64,<payload>`
  - 其次 `data[0].url`

为了保持改动面小，不实现 multipart 上传、mask、批量多图输出、质量参数映射或模型专属参数。

## 错误处理

- 前端按钮禁用负责防止无参考图编辑。
- 后端仍校验编辑参考图不能为空，避免被直接命令调用绕过。
- 网络错误沿用 `reqwest.error_for_status()` 的现有风格。
- 响应结构错误返回可读 provider 错误，但不包含完整 base64。

## 测试策略

- TypeScript：
  - `customProviderConfig.test.ts` 覆盖协议归一化、配置校验。
  - `runtimeRegistry.test.ts` 覆盖 `openai-image` 运行时协议透传。
  - `ImageEditNode.test.tsx` 覆盖协议匹配时显示编辑按钮、无参考图禁用、有参考图点击后提交 `action: "edit"`。
- Rust：
  - `openai_image` 模块单测覆盖生成请求体、编辑请求体、响应解析和缺少参考图错误。
  - `commands::ai` 中协议判断可用纯函数测试覆盖 `openai-image` 分流。

## 官方契约依据

- OpenAI Image generation guide: `https://developers.openai.com/api/docs/guides/image-generation?api=image`
- OpenAI Images API reference: `https://platform.openai.com/docs/api-reference/images`
