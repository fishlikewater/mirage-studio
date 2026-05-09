# 验证与发布规范

## 常用开发命令

```bash
npm run dev
npm run tauri dev
```

## 快速检查

```bash
npx tsc --noEmit
cd src-tauri && cargo check
```

Rust 未变更时可跳过 `cargo check`，但必须说明原因。

## 收尾检查

```bash
npm test
npm run build
```

影响依赖、入口、持久化、Tauri 命令或发布链路时，应运行更完整的构建或联调验证。

## 发布快捷口令

当用户明确说“推送更新”时，默认执行一次补丁版本发布：

- 基于上一个 release/tag 自动递增 patch 版本号。
- 汇总代码变动生成 Markdown 更新日志。
- 完成版本同步、发布提交、annotated tag 与远端推送。
- 如用户额外指定 minor、major 或自定义说明，则按用户要求覆盖默认行为。

自动生成的更新日志正文只保留 `## 新增`、`## 优化`、`## 修复` 等二级标题分组与对应列表项；不要额外输出 `# vx.y.z` 标题、范围说明或 `## 完整提交` 区块，空分组可省略。

## 提交前检查清单

- 功能路径可用，至少手测 1 条主路径和 1 条异常路径。
- 无明显性能回退，重点关注拖拽、缩放、输入响应。
- 轻量检查通过：`npx tsc --noEmit`，Rust 改动则 `cargo check`。
- 大改或发布前运行 `npm run build`。
- 正式发布前确认 `docs/releases/vx.y.z.md` 已更新，并与本次 tag/版本号一致。
- 新增约束或行为变化已同步 `.trellis/spec/` 或 OpenSpec。
