# 后端质量规范

## 完成定义

至少满足：

- 编译 / 构建通过
- 测试通过
- 核心场景已验证
- 新增约定已沉淀到 spec

## 推荐门禁

- 静态检查：`cd src-tauri && cargo check`
- 单元测试：按改动范围运行 `cd src-tauri && cargo test 具体测试名`，或直接运行完整 `cargo test`
- 集成/前后端契约验证：涉及 Tauri 命令时同步运行相关前端测试或手测主路径
- 构建验证：影响入口、依赖、打包或发布链路时运行 `npm run build`，必要时运行 `npm run tauri build`

## 常见禁止模式

- 未验证就宣称完成
- 用硬编码绕过配置与契约
- 用临时兼容逻辑代替真实修复
- 变更接口或数据结构却不更新文档和调用方
- Rust provider 注册、前端 provider registry、设置页 API Key 链路不同步
- SQLite schema 只改读取/写入实现，未改自愈建表逻辑
