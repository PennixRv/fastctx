# Control-plane 基线

- `src/cli/mod.rs:44-71` 只暴露 `apply`、`unapply` 和 `status`；当前没有 guidance mode 或独立命令。
- `src/control/apply.rs:329-541` 生成包含 Codex config、`AGENTS.md`、binary 与 receipt 的单个 ApplyPlan；`src/control/apply.rs:585-721` 生成完整 UnapplyPlan。
- `src/control/codex_config.rs:62-127` 独立编辑 MCP、namespace 和 token limit；它是 FastCtx runtime 的类型化配置所有者。
- `src/control/agents.rs` 已提供 exact apply/remove/classify；补丁应复用它，而非以字符串脚本改用户文件。
- 版本基线来自 `Cargo.toml:3` 和上游 GitHub `v0.2.6` release（2026-08-23）。
