# 实施计划

1. 阅读 control/backend spec、task artifacts 和现有 receipt/doctor 测试，确定 `AppliedRecord` 的兼容序列化方案。
2. 先添加 receipt guidance ownership 类型与纯计划函数测试；覆盖 absence、current、foreign 和 invalid 状态。
3. 实现 `GuidanceMode` 及 `apply --guidance`；默认路径复用既有 managed 分支，`none` 不形成 AGENTS file change。
4. 实现 `guidance status|apply|remove`，复用 immutable preview、transaction 和 TTY/`--yes` 确认逻辑。
5. 调整 doctor/status 和 `unapply`，确保 disabled absence 通过、managed drift 失败、Pennix marker 不受影响。
6. 补 CLI golden/contract 与跨平台路径测试；运行格式化、聚焦测试和完整 `cargo test`。
7. 更新版本、README/release notes 和 npm package identity；提交、推送 `PennixRv/fastctx`，创建修正版 `v0.2.9` GitHub release，并由 Actions 真实发布 scoped npm 包，记录资产、registry 版本与 commit。
8. 在 `pennix-skills` 将 FastCtx catalog 接入 npm installer，完成测试、提交、推送，并通过 collection installer 安装。
9. 运行 Bootstrap 的 discover/plan/confirmed apply，完成 `/home/penn/.codex` 静态模板与 MCP 验收；验证默认 Apply/TUI 不写 `AGENTS.md`。
10. 更新根仓库 rollout 任务并完成跨组件验收、提交和归档。

## Validation

```text
cargo fmt --check
cargo test --test cli_apply_golden
cargo test --test cli_contract
cargo test
cargo build --release
gh release view v0.2.9 --repo PennixRv/fastctx
npm view @pennixrv/fastctx@0.2.9 version --registry https://registry.npmjs.org/
```

## Rollback

- 在 release 前，丢弃未发布的 fork branch 并保持 upstream-compatible `v0.2.6` 基线。
- host 迁移失败时，不执行完整 `unapply`；由后续 host 任务按 preview 先恢复 FastCtx marker，再决定是否撤回 MCP。
- release 发现控制面回归时，发布修正 patch，不重写或移动已发布 tag。
