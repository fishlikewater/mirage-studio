# OpenAI Image Protocol

## Goal

为自定义供应商新增 `openai-image` 接入协议，支持 OpenAI Images API 的生成与显式编辑调用，并在图像节点中按当前供应商协议显示“编辑”按钮。

## Requirements

- 自定义供应商协议支持 `openai-image`。
- `openai-image` 使用 Base URL、API Key 和模型 ID 配置。
- 图像节点选择 `openai-image` 模型时，保留现有“生成”按钮，并在旁边显示“编辑”按钮。
- “编辑”按钮复用现有 `@` 指定关联节点图片的逻辑，不新增独立选图入口。
- 没有通过 `@` 或图谱输入解析出参考图时，“编辑”按钮禁用。
- 生成调用 OpenAI Images generation endpoint。
- 编辑调用 OpenAI Images edit endpoint，并把现有 `referenceImages` 作为参考图输入。
- 响应优先解析 `data[0].b64_json`，兼容 `data[0].url`。

## Acceptance Criteria

- [ ] 设置页可选择并保存 `OpenAI Image` 接入协议。
- [ ] 运行时模型配置透传 `providerRuntime.protocol = "openai-image"`。
- [ ] 图像节点在 `openai-image` 模型下显示“生成”和“编辑”两个按钮。
- [ ] 无参考图时“编辑”按钮禁用。
- [ ] 有 `@` 关联参考图时点击“编辑”提交 `action = "edit"` 和现有 `referenceImages`。
- [ ] 后端 `generate` 动作调用 `/images/generations`。
- [ ] 后端 `edit` 动作调用 `/images/edits`。
- [ ] 相关 TypeScript 与 Rust 测试通过。

## Technical Notes

- Change: `.cowork-flow/changes/openai-image-protocol/`
- Plan: `.cowork-flow/plans/2026-05-21-openai-image-protocol.md`
- 分级：L2，涉及前端设置、节点 UI、Tauri DTO、Rust 后端协议适配。
- 实现方式：TDD，先写失败测试，再写最小实现。
