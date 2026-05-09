# 协作约定

请与 `.trellis/spec/`、`.trellis/workflow.md`、`openspec/` 中的项目规范合并使用；如有冲突，以用户明确指令和运行时安全要求为最高优先级。

## 0. 项目定制位

- 项目名称：`StoryBoard-Copilot`
- 产品目标：基于节点画布进行图片上传、AI 生成/编辑、工具处理、裁剪、标注和分镜。
- 主要技术栈：React、TypeScript、Zustand、@xyflow/react、TailwindCSS、Tauri 2、Rust、SQLite/rusqlite。
- 主要运行命令：`npm run dev`、`npm run tauri dev`
- 主要测试命令：`npx tsc --noEmit`、`npm test`、`cd src-tauri && cargo check`
- 提交策略：混合；普通开发按当前会话约定执行，用户明确要求提交、发布或推送时可由 AI 执行对应 git/release 流程。
- 文档语言：中文。
- Skill 语言：修改已有 `SKILL.md` 时，新增内容跟随原文件语言，不做中英混写。
- 详细技术规范入口：`.trellis/spec/index.md`

---

## 1. 编码前先思考

**不要想当然，不要掩饰困惑，要把假设和取舍摆到台面上。**

- 先明确写出自己的假设；不确定时就提问。
- 如果需求存在多种解释，先把几种理解列出来，不要静默替用户做决定。
- 如果存在更简单的做法，要主动指出。
- 需要时可以温和地提出异议，不盲从执行。
- 一旦发现信息不清、边界模糊或描述矛盾，先停下来，说明困惑点并澄清。

## 2. 简单优先

**用最少的代码解决问题，不做投机式设计。**

- 不添加用户没有要求的功能。
- 不为一次性代码提前做抽象。
- 不加入未被要求的“灵活性”“可配置化”“通用化”。
- 不为明显不可能发生的场景堆砌错误处理。
- 如果写了很多代码，但更少代码能清楚解决问题，就应继续简化。

## 3. 外科手术式改动

**只改必须改的地方，只清理由自己改动带来的问题。**

- 不顺手重构无关模块。
- 不为“顺便优化”扩大改动面。
- 尽量贴合项目现有结构、命名和风格。
- 只删除因为本次修改而成为孤儿的代码。
- 不覆盖用户已有改动；发现冲突先停下来说明。

## 4. 以目标驱动执行

**先定义成功标准，再循环验证直到目标达成。**

把任务改写成可验证目标：

- “修 bug” -> “先复现，再补回归验证，再修复”
- “加能力” -> “先明确输入输出，再实现，再验证”
- “重构” -> “先确认行为基线，再保证改前改后一致”

多步骤任务建议使用：

```text
1. [步骤] -> 验证：[检查项]
2. [步骤] -> 验证：[检查项]
3. [步骤] -> 验证：[检查项]
```

## 5. 规格与执行协作

**有行为变化的任务，统一采用 `OpenSpec + superpowers + Trellis` 协作流。**

- `L0`：纯文档、测试、重构、工具配置或不改变用户可见行为的调整，可直接进入 Trellis 执行。
- `L1`：单模块或单能力的用户可见行为变化，先补 OpenSpec proposal/spec，再写实现计划。
- `L2`：跨模块、跨层、接口契约、持久化结构、模型供应商协议或架构边界变化，必须补 OpenSpec proposal/spec/design，并在实现后完成更严格 review。
- 方案讨论使用 `superpowers:brainstorming`，开发计划必须显式使用 `superpowers:writing-plans`。
- Trellis 负责执行过程、上下文同步、journal 留痕与多 Agent 协作。

## 6. 项目规范入口

动手前按任务范围读取：

- 项目总览：`.trellis/spec/project-overview.md`
- 前端规范：`.trellis/spec/frontend/index.md`
- 后端规范：`.trellis/spec/backend/index.md`
- 思考指引：`.trellis/spec/guides/index.md`
- 主流程：`.trellis/workflow.md`
- 行为变更：`openspec/`

常用上下文命令：

```bash
python3 ./.trellis/scripts/get_context.py
python3 ./.trellis/scripts/task.py list
```

## 7. Skill 修改约束

- `SKILL.md` 应描述可复用的流程、判定规则、检查点与命令占位，不应写成某次任务的临时施工单。
- 除非该 skill 明确就是当前项目专用能力，否则不要写入具体业务模块名、一次性任务复选项或缺乏泛化价值的文件路径。
- 需要举例时，优先使用目录类型、场景类型或占位路径，而不是直接写某个项目独有文件。
- 修改已有 skill 时，保持原文件语言一致：英文文件追加英文，中文文件追加中文；如需切换语言，应整体统一改写。

## 8. 提交与验证口径

- 业务代码由当前会话约定的执行者提交。
- `.trellis/workspace`、`.trellis/tasks` 等元数据优先由脚本自动提交。
- 不要在未验证的情况下声称“已完成”“已通过”“可交付”。
- 未执行的验证命令必须说明原因。

<!-- TRELLIS:START -->
# Trellis 说明

以下说明供在本项目中工作的 AI 助手使用。

开始新会话时，使用本地 `$start` skill 或以下命令来：
- 初始化开发者身份
- 理解当前项目上下文
- 读取相关规范

```bash
python3 ./.trellis/scripts/get_context.py
python3 ./.trellis/scripts/task.py list
```

使用 `@/.trellis/` 了解：
- 开发工作流：`workflow.md`
- 项目结构与技术规范：`spec/`
- 开发者工作区：`workspace/`

保留此管理块，便于 Trellis 工具后续刷新说明。

<!-- TRELLIS:END -->

---

如与用户明确要求冲突，以用户要求优先；如与运行时安全冲突，以安全优先。
