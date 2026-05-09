# 思考指引

> 本目录不讲实现细节，只提醒最容易出问题的思考点。

---

## 常见风险

- 模块边界被打穿
- 节点 registry、节点类型、节点渲染组件只改了一部分
- Tauri 命令、前端命令封装、调用方契约只改了一半
- 新增一套重复常量、错误码、查询模式
- 图片字段、持久化结构、模型 provider 协议没有同步更新
- i18n 只新增一种语言或使用不稳定 key

---

## 使用方式

- 写代码前先看 [编码前检查表](./pre-implementation-checklist.md)
- 涉及跨模块、异步链路、权限或状态流转时，看 [跨层思考指引](./cross-layer-thinking-guide.md)
- 涉及公共能力、常量、错误码、查询模板、校验逻辑时，看 [代码复用思考指引](./code-reuse-thinking-guide.md)
- 涉及模块边界、文件规模或层间职责时，看 [架构与边界规范](./architecture-boundaries.md)
- 涉及行为变更流程时，看 [OpenSpec、superpowers 与 Trellis 协作规范](./spec-collaboration.md)
- 涉及发布、构建、提交前门禁时，看 [验证与发布规范](./release-and-verification.md)

---

## 最小动作

修改前先搜索：

```bash
rg "关键字"
```
