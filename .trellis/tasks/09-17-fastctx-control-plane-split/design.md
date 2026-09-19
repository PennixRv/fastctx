# FastCtx MCP 与 guidance 控制面分离设计

## Public CLI

```text
fastctx apply [--guidance managed|none] [--codex-home PATH] [--tier TIER] [--yes]
fastctx guidance status [--codex-home PATH]
fastctx guidance apply [--codex-home PATH] [--yes]
fastctx guidance remove [--codex-home PATH] [--yes]
```

- `apply` 没有 `--guidance` 时默认只配置 MCP、binary 和 receipt，不读写 `AGENTS.md`；这是 Pennix v0.2.8 的默认行为。
- `apply --guidance none` 显式保持同一不写入 `AGENTS.md` 的行为；`managed` 仅供明确旧安装兼容，不由 Pennix Bootstrap 调用。
- `guidance apply`/`remove` 只编辑 FastCtx marker，绝不触碰 Codex config、binary、tool budget 或 Pennix marker；仅当已有 Apply receipt 且 profile 匹配时工作。
- 所有会写的 guidance 命令沿用 Apply 的 preview/TTY/`--yes` 合同，并使用事务层的 compare-before-commit 保护。

## Receipt And Doctor Contract

新增 receipt 的 optional `guidance` record，保存 FastCtx marker 的应用 hash、原文件存在性、插入分隔符和 contract ID。旧 receipt 解码为 `managed` 兼容状态。

| Receipt / file state | `fastctx status` guidance 结果 | 说明 |
| --- | --- | --- |
| guidance record + exact current marker | `PASS current` | 默认 Apply 或 `guidance apply`。 |
| no guidance record + marker absent | `PASS absent (disabled)` | 已显式选择 `apply --guidance none` 或 `guidance remove`。 |
| guidance record + marker missing/foreign/invalid | `FAIL` | 已管理内容发生 drift；不自动修复。 |
| no guidance record + FastCtx marker exists | `FAIL foreign` | 不接管未知/遗留内容；用户必须明确处理。 |

`unapply` 保持完整卸载：仅在 receipt 有 guidance record 时移除 FastCtx marker；无 record 时不访问 `AGENTS.md`。这样它不可能删除之后由 Pennix setup 写入的不同 marker。

## Rollout Boundary

- `src/cli/mod.rs`：解析 `GuidanceMode` 与 `Guidance` 子命令，复用 preview/confirmation 输出。
- `src/control/apply.rs`：拆出 MCP/binary/receipt plan 与 guidance plan，保留单次 `apply` 的原子组合提交；为独立 guidance 操作提供 immutable plan/commit。
- `src/control/settings.rs`：在 `AppliedRecord` 增加可选 FastCtx guidance ownership record，并保留旧 receipt 兼容。
- `src/control/doctor.rs`：将 MCP 与 guidance 分项检查；absence 仅在 receipt 明确禁用时通过。
- `tests/`：扩展 apply golden 与 CLI/doctor contract；加入 exact marker 拒绝、none、standalone apply/remove 和 unapply 不删 Pennix marker 的覆盖。
- `Cargo.toml`、README、release notes：更新版本/发布身份与新命令文档。
- `pennix-skills`：新增 `pennix-fastctx-setup`（显式部署）与 `pennix-fastctx-routing`（按需路由）两个直接 Skill；不将 FastCtx runtime 纳入 submodule。
- `/home/penn/.codex`：仅经 Bootstrap 的 plan/confirmed apply 写入 MCP 与静态 Pennix 模板；不再使用 FastCtx 自有 guidance marker。
- 根仓库：记录组件提交/release 与 host 验收，作为 rollout 的集成事实。

## Deliberate Non-Goals

- FastCtx runtime 不直接管理 Pennix marker、Skill、项目 `AGENTS.md` 或 `pennix-skills` 安装；这些由本 rollout task 中的 setup Skill 负责。
- 不添加 `mcp-only` 产品 profile；那是 Pennix host 层已否决的选择，不属于 FastCtx CLI。
- 不改 MCP runtime、tool schema、job 语义、provider、TUI 交互或 updater 机制。

## Release Contract

以 upstream `v0.2.6` package/API 作为基线，Pennix fork 发布修正版 tag `v0.2.10`。Cargo package 的 repository、authors 和 package identity 使用 fork 可追溯信息；npm 发布使用 `@pennixrv/fastctx`、`@pennixrv/codex-fastctx` 和对应 platform 包，不占用官方 unscoped 名称。本 rollout task 的 Bootstrap 必须使用 npm registry 的精确版本，不从本地 build 猜测版本。
