# 开发工作流

> 本文件是 StoryBoard-Copilot 的 Trellis 主流程。它保留 workflow-starter 的流程骨架，但所有命令、分级和门禁均以当前项目事实为准。

## 1. 当前项目事实

- 仓库形态：前端 + Tauri/Rust 后端的混合桌面应用。
- 前端：React、TypeScript、Zustand、@xyflow/react、TailwindCSS、i18next。
- 后端：Tauri 2、Rust、SQLite/rusqlite、WAL。
- 核心业务：节点画布、图片上传、AI 生成/编辑、工具处理、分镜与项目持久化。
- 主要规范入口：`AGENTS.md`、`.trellis/spec/`、`openspec/`、`docs/superpowers/`。

## 2. 快速开始

首次使用 Trellis 时初始化开发者身份：

```bash
python3 ./.trellis/scripts/get_developer.py
python3 ./.trellis/scripts/init_developer.py <your-name>
```

每次会话开始先获取上下文：

```bash
python3 ./.trellis/scripts/get_context.py
python3 ./.trellis/scripts/task.py list
git status
git log --oneline -10
```

动手前至少阅读：

```bash
Get-Content AGENTS.md
Get-Content .trellis/spec/index.md
Get-Content .trellis/spec/frontend/index.md
Get-Content .trellis/spec/backend/index.md
Get-Content .trellis/spec/guides/index.md
```

在类 Unix shell 中可用 `cat` 替代 `Get-Content`。

## 3. 变更级别

| 级别 | 适用场景 | 必要流程 |
|------|----------|----------|
| `L0` | 文档、测试、内部重构、工具配置、无用户可见行为变化 | 直接创建/选择 Trellis 任务，说明验证方式后执行 |
| `L1` | 单模块或单能力行为变化，例如新增节点局部交互、调整工具参数、改变局部 UI 行为 | 先补 OpenSpec proposal/spec，再写计划并执行 |
| `L2` | 跨层/跨模块变化，例如节点数据结构、持久化 schema、Tauri 命令、模型供应商协议、画布核心数据流 | 补 OpenSpec proposal/spec/design，计划后执行，并进行更严格 review |

判断不确定时，按更高一级处理；这比事后补规格更稳。

## 4. L0 执行流程

1. 明确目标、范围和验证命令。
2. 创建或选择 Trellis 任务：

```bash
python3 ./.trellis/scripts/task.py create "<title>" --slug <task-name>
python3 ./.trellis/scripts/task.py start <task-dir>
```

3. 阅读相关 `.trellis/spec/` 文档与现有实现。
4. 小步实现，不扩大无关改动。
5. 运行与改动范围匹配的验证命令。
6. 完成后记录 session。

## 5. L1/L2 行为变更流程

1. 创建 OpenSpec change，并补齐 proposal/spec；`L2` 还必须补齐 design。
2. 校验 OpenSpec：

```bash
openspec validate --strict --type change <slug>
```

3. 使用 `superpowers:brainstorming` 对齐方案，使用 `superpowers:writing-plans` 产出实现计划。
4. 检查 `docs/superpowers/plans/YYYY-MM-DD-<slug>.md` 是否包含可回写 checkbox 和“当前执行状态”区块。
5. 将高层里程碑同步到 `openspec/changes/<slug>/tasks.md`。
6. 创建或绑定 Trellis 任务，把 proposal/spec/design/plan 加入任务上下文。
7. 执行实现、验证、review；每个步骤完成且验证通过后，及时更新计划 checkbox 和当前执行状态。
8. 完成后归档 OpenSpec change，并记录 session。

如果本机没有 OpenSpec CLI，不要假装校验已完成；应说明阻塞，并用文档审阅替代到 CLI 可用为止。

## 6. 项目验证命令

常用轻量验证：

```bash
npx tsc --noEmit
cd src-tauri && cargo check
```

前端测试：

```bash
npm test
```

完整前端构建：

```bash
npm run build
```

Tauri 联调与打包按需执行：

```bash
npm run tauri dev
npm run tauri build
```

发布入口：

```bash
npm run release -- patch --notes-file docs/releases/vx.y.z.md
```

说明：Rust 未变更时可跳过 `cargo check`；影响依赖、入口、持久化、Tauri 命令或发布链路时，应运行更完整的构建/联调验证。

## 7. 任务上下文

Trellis 任务目录用于沉淀任务目标、上下文和执行状态：

```text
.trellis/tasks/
├── <task-name>/
│   ├── task.json
│   ├── prd.md
│   ├── implement.jsonl
│   ├── check.jsonl
│   └── debug.jsonl
└── archive/
```

将相关规范加入上下文：

```bash
python3 ./.trellis/scripts/task.py init-context <task-dir> fullstack
python3 ./.trellis/scripts/task.py add-context <task-dir> implement ".trellis/spec/frontend/index.md" "前端规范入口"
python3 ./.trellis/scripts/task.py add-context <task-dir> check ".trellis/spec/guides/pre-implementation-checklist.md" "收尾检查"
```

## 8. 会话结束

完成任务后记录 session：

```bash
python3 ./.trellis/scripts/add_session.py --title "Session Title" --commit "abc1234" --summary "Brief summary"
```

结束前确认：

- 验证命令已按范围执行，未执行的命令已说明原因。
- 行为变更对应的 OpenSpec、计划、任务状态已同步。
- `docs/superpowers/plans/*.md` 中的 checkbox、当前执行状态、验证/review 结论与真实进度一致。
- `.trellis/spec/` 中需要沉淀的新规则已更新。
- 工作区中无误删、无与本次任务无关的回退。

## 9. 最佳实践

- 先读规范再编码，先搜索现有实现再新增能力。
- 行为变更先规格、再计划、再实现。
- Store 不承载重业务逻辑，跨层数据流按 `UI -> Store -> 应用服务 -> Tauri 命令/API -> 持久化` 检查。
- 修改 `SKILL.md` 时保持语言一致；通用 skill 写场景规则和检查点，不写一次性任务私货。
- 画布交互优先保障流畅，不在拖拽/缩放每帧执行重持久化。
- 完成定义必须包含验证证据，而不是“看起来没问题”。
