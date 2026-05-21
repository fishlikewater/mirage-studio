# Custom Provider OpenAI Image Protocol Spec

## 外部行为

### 协议配置

- 自定义供应商的接入协议列表必须包含 `openai-image`。
- `openai-image` 使用与 OpenAI Images API 兼容的 Base URL 和 API Key。
- 配置字段：
  - `baseUrl`：例如 `https://api.openai.com/v1`。
  - `apiKey`：Bearer token。
  - `model.remoteModelId`：发送给 OpenAI Images API 的模型名，例如 `gpt-image-1.5`、`gpt-image-1` 或兼容服务的模型 ID。
- 旧配置中缺失协议或使用未知协议时，仍按现有默认 `openapi` 归一化，不能破坏历史数据。

### 生成行为

- 当节点选择的模型来自 `openai-image` 自定义供应商，点击现有“生成”按钮时：
  - 前端提交 `providerRuntime.protocol = "openai-image"`。
  - 后端调用 `{baseUrl}/images/generations`。
  - 请求体至少包含：
    - `model`
    - `prompt`
    - `n: 1`
  - 如果节点选择了尺寸或比例，可将当前项目的比例信息附加到 prompt，保持与现有自定义 OpenAPI 行为一致。
  - 响应优先读取 `data[0].b64_json`，返回 `data:image/png;base64,<payload>` 给现有图片持久化流程。
  - 如果兼容服务返回 `data[0].url`，后端可以直接返回 URL。

### 编辑行为

- 当节点选择的模型来自 `openai-image` 自定义供应商时，图像节点操作区必须在“生成”按钮旁显示“编辑”按钮。
- “编辑”按钮使用现有 `@` 指定关联节点图片的逻辑：
  - 用户在当前节点提示词中用 `@` 关联图片节点、上传节点或结果图片节点。
  - 现有引用解析链路得到 `referenceImages`。
  - 编辑动作把这些 `referenceImages` 作为 OpenAI 编辑接口的参考图。
- 当 `referenceImages` 为空时，“编辑”按钮必须禁用。
- 点击“编辑”时：
  - 前端提交同一个生成 payload，但必须显式标记动作为 `edit`。
  - 后端调用 `{baseUrl}/images/edits`。
  - 请求体使用 JSON 形式，至少包含：
    - `model`
    - `prompt`
    - `images: [{ image_url: <data-url-or-url> }, ...]`
    - `n: 1`
  - 响应解析规则与生成一致，优先 `data[0].b64_json`，兼容 `data[0].url`。

### 错误行为

- 缺少 Base URL、API Key 或模型 ID 时，错误信息必须指向 `custom openai-image provider`。
- 编辑动作缺少参考图时，后端必须返回明确错误；前端按钮禁用是第一层保护。
- OpenAI 响应缺少可用图片结果时，错误信息必须说明缺少 `data[0].b64_json` 或 `data[0].url`。
- 不记录完整 API Key 和完整 base64 图片内容到前端日志。

## 验收标准

- [ ] 设置页可选择 `OpenAI Image` 接入协议，并保存/重新打开不丢失配置。
- [ ] 运行时模型列表能把 `openai-image` 自定义模型映射为 `providerRuntime.protocol = "openai-image"`。
- [ ] 选择 `openai-image` 模型时，图像节点显示“生成”和“编辑”两个动作。
- [ ] 没有通过 `@` 关联参考图时，“编辑”按钮禁用。
- [ ] 有通过 `@` 关联参考图时，点击“编辑”会提交编辑动作，并把关联图片传给后端。
- [ ] 后端生成请求使用 `/images/generations`。
- [ ] 后端编辑请求使用 `/images/edits`，并用 JSON `images` 字段表达参考图。
- [ ] `b64_json` 响应转换为 `data:image/png;base64,...` 后能被现有图片持久化流程处理。
- [ ] 既有 `openapi` 与 `xais-task` 测试保持通过。
