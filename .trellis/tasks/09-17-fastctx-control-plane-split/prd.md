# 分离 FastCtx MCP 与 guidance 控制面

## Goal

完成 FastCtx guidance 解耦的完整 Pennix rollout：在 `PennixRv/fastctx` 分离 Codex MCP 与 FastCtx guidance 控制面，发布 scoped npm 包族和 GitHub 资产修正版 `v0.2.11`；在 `pennix-skills` 通过 Bootstrap 完成安装与静态 guidance 管理，并完成 `/home/penn/.codex` 的集成验收。

## Confirmed Facts

- `src/control/apply.rs` 的 `plan_apply` 当前在一个不可变事务中同时写入 binary、Codex MCP config、FastCtx marker guidance 和 receipt；`plan_unapply` 同样完整移除这些目标。
- `src/control/agents.rs` 已具有精确 apply/remove/classify 能力；控制面缺口是 CLI、receipt 与 doctor 没有表达“guidance 有意禁用”。
- `src/control/codex_config.rs` 已独立处理 MCP server、预算、namespace 和 shared host limit，因此不能由 Pennix 脚本复制其语义。
- 用户已批准此 fork、发布、Pennix Skills 更新和 guided host 迁移；本任务是这些变更的唯一 rollout task。

## Requirements

- 新增公开 `--guidance <managed|none>` 到 `fastctx apply`；省略时默认为 `none`，正常 Apply/TUI 不读取或写入用户 `AGENTS.md`；`managed` 仅保留为显式旧安装兼容路径。
- 新增 `fastctx guidance status|apply|remove`，每个操作可指定 `--codex-home`；会写文件的 `apply`/`remove` 必须提供不可变 preview，且仅在 `--yes` 或 TTY 明确确认后提交。
- 将 receipt 和 doctor 重构为分别追踪 MCP 配置和 guidance 状态；`none`/`absent` 是显式健康状态，不得被报告成 drift 或要求重跑默认 Apply。
- 所有 guidance 写入或删除只操作 FastCtx 精确 marker；重复、嵌套、未知或畸形 marker 必须拒绝，不得覆盖用户或 Pennix marker。
- 保持 `fastctx unapply` 的完整卸载语义：删除 FastCtx MCP、FastCtx-owned guidance（若存在）、receipt、binary 与 runtime state；不得删除 Pennix-owned marker。
- 版本、README/release metadata 和 package identity 明确标识 `PennixRv/fastctx` 及修正版 `v0.2.11`；npm 包统一使用 `@pennixrv/fastctx*`，不得占用或伪装为官方上游包名。
- 发布 workflow 必须先验证 platform/main/alias npm tarball，再按 platform → main → alias 顺序发布；使用 `NPM_TOKEN` secret 和 npm provenance，不把 token 写入仓库。
- 在 `pennix-skills` 由 `pennix-workflow-bootstrap` 统一承载安装、静态模板和 project-init 路由；FastCtx 不再需要独立 setup 入口。
- 使用发布的 scoped fork 精确版本安装/配置 FastCtx；Pennix Bootstrap 的静态 `AGENTS.md` 模板是唯一 workflow guidance 来源，FastCtx 不得自动注入或刷新用户 `AGENTS.md`。
- 在根仓库记录跨组件 release/commit、迁移和路由验收证据，完成后关闭所有本次创建的 rollout 记录。

## Acceptance Criteria

- [ ] `apply` 默认不写入 `AGENTS.md`；显式 `--guidance managed` 的旧兼容路径仍通过 contract 测试。
- [ ] `apply --guidance none` 只处理 MCP/binary/receipt，不添加 FastCtx marker，并将 absence 记录为健康状态。
- [ ] `guidance status|apply|remove` 覆盖 absent、current、known legacy、foreign/invalid marker 和 profile mismatch，且 preview 与 commit 使用同一不可变变更集。
- [ ] `status` 分别报告 MCP 配置、runtime、guidance 状态；在 configured-none 状态下整体诊断可通过。
- [ ] 单元/CLI golden/contract 测试覆盖控制面分离、默认兼容、回滚和未知 marker 拒绝；现有全套 Rust 测试通过。
- [ ] `Cargo.toml`、README、release notes、npm registry 和 GitHub release 形成可核验的 Pennix `v0.2.11` 发布；源码和发行资产都可追溯到同一 commit。
- [ ] `pennix-fastctx-setup` 与 `pennix-fastctx-routing` 已在 `pennix-skills` 实现、测试、提交、推送并安装至用户级发现根。
- [ ] 静态 Bootstrap guidance 经模板验收后存在；FastCtx 默认 Apply/TUI 不新建、修改或刷新用户 `AGENTS.md`。
- [ ] 根仓库已记录并核验 release/commit、MCP tools-list、routing 样例和回滚路径；所有工作树干净。

## Notes

- 本任务的修改目标包括 `/home/penn/devel/fastctx` 与 `PennixRv/fastctx` release、`/home/penn/devel/codex-workflow-optimization/pennix-skills`、`/home/penn/.codex` 静态配置/Skill 安装，以及根仓库的验收记录。每个 Git 仓库保持独立提交。
- Shell/job/runtime/MCP 工具 schema 不在本次改动范围；控制面补丁不得改变它们的行为。
- 发布后由 `pennix-workflow-bootstrap` 消费该 scoped npm release；已配置的 `NPM_TOKEN` 仅供 GitHub Actions 使用。

## Release Repair

- `v0.2.8` 的远端 tag 已存在但发布 workflow 失败；GitHub Release 与 npm 包均未创建。
- 失败原因是 publish job 的无筛选 `download-artifact` 将 `npm-*` tarball 一并下载到 `dist/release`，触发归档集合校验失败。
- 修复为 `pattern: build-*`，并发布不可变修正版 `v0.2.9`，不重写 `v0.2.8` tag。
- `v0.2.9` 的构建和归档校验已通过，但 `verify-npm-install.ps1` 仍用未 scoped 的 `fastctx` 路径查找主包 launcher，导致 npm 安装合同失败。
- 修复主包 launcher 路径为 `@pennixrv/fastctx`，并发布不可变修正版 `v0.2.10`，不重写既有 tag。
- `v0.2.10` 的 npm 安装路径已通过，但 `verify-launcher-lifecycle.js` 的 provenance 合同仍期待未 scoped 的 `fastctx` 包名，导致发布前生命周期校验失败。
- 将 provenance 合同改为 `@pennixrv/fastctx`，并发布不可变修正版 `v0.2.11`，不重写既有 tag。
