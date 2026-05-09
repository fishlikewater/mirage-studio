# 前端质量规范

## 完成定义

至少满足：

- 构建通过
- 类型检查通过
- lint / format 通过
- 关键交互已手工验证

## 推荐门禁

- lint：当前项目未配置独立 lint 脚本，不要虚构通过状态。
- type-check：`npx tsc --noEmit`
- test：`npm test`
- build：`npm run build`
- Rust 相关改动：`cd src-tauri && cargo check`

## 常见禁止模式

- 用 `any` 或强制断言掩盖真实类型问题
- 组件未处理加载、空态、错误态
- 交互依赖隐式副作用
- 新增文案只改一种语言包
- 新增节点/工具只改 UI，未同步 registry、类型守卫或处理链路
