# openai-image-protocol

## 背景

当前自定义供应商支持 `openapi` 与 `xais-task` 两类接入协议，但 OpenAI Images API 的契约与这两类都不同：

- OpenAI 图像生成使用 `POST /v1/images/generations`，结果通常从 `data[0].b64_json` 读取。
- OpenAI 图像编辑使用 `POST /v1/images/edits`，参考图通过请求体中的 `images` 传入。
- 现有 `openapi` 协议面向 chat completions，并从 markdown 图片链接中提取结果，不能稳定表达 OpenAI Images API。

因此需要新增一个专门的 `openai-image` 接入协议。

## 目标

- 在自定义供应商配置中新增 `openai-image` 协议。
- 支持 OpenAI Images API 的生成与编辑两个动作。
- 在图像生成节点中，当当前模型来自 `openai-image` 供应商时，在生成按钮旁显示“编辑”按钮。
- “编辑”按钮复用现有 `@` 关联节点图片解析结果作为参考图，不新增独立选图逻辑。

## 非目标

- 不新增内置 OpenAI 供应商。
- 不修改已有 `openapi` / `xais-task` 协议语义。
- 不引入多结果选择、遮罩编辑或文件上传管理。
- 不改变普通“生成”按钮对其他供应商的行为。

## 成功标准

- 用户可以创建自定义供应商，选择 `OpenAI Image` 协议，填写 Base URL、API Key 和模型 ID。
- 使用该供应商模型点击“生成”时，调用 OpenAI 图像生成接口。
- 使用该供应商模型且当前节点通过 `@` 关联到至少一张图片时，点击“编辑”调用 OpenAI 图像编辑接口。
- 没有参考图时，“编辑”按钮不可用，避免发出无效编辑请求。
- 相关前端单测和 Rust 后端单测覆盖协议归一化、运行时配置、请求体构造和响应解析。
