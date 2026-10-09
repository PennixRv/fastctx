# Timeout representation reconciliation

Root coordination: `codex-workflow-optimization/.trellis/tasks/10-09-workflow-footprint-reconciliation`.

- Fixed the shared applied-state checker, preserving exact timeout values and all other receipt/ownership checks. Integer and equivalent float TOML values now match; fractional changes, strings, `nan` and `inf` remain drift.
- `cargo test --locked --test cli_contract`: all 10 tests passed. The regression runs the real isolated apply/status/unapply route with seven positive/negative configurations.
- `cargo fmt --check` and `git diff --check`: passed.
- `cargo test --locked --lib control::`: all 9 shared control tests passed; workflow provenance verified at the explicit immutable commit.
- The initial `v0.2.22` release matrix built all platform artifacts, but the final distribution guard falsely rejected the tracked lowercase Trellis `agents.md`; no release artifacts were published. The pushed tag remains immutable.
- Retargeted the coordinated release to `0.2.23`, fixing the guard with case-sensitive private-path checks. Cargo package/lock and every npm platform/alias manifest are aligned; allocator-api2 remains `0.2.21`.
- Native project update moved generated Skills to beta.41. Explicit native workflow refresh adopted the same reviewed Marketplace commit as the root (`85d17acef228127cabe62bc409a16b6cf24f1d29`); preserved project-specific assets.
- `cargo fmt --check` and `cargo test --locked --test cli_contract` pass (10 tests). Local PowerShell is unavailable, so the distribution script and exact npm tarball contract remain for release CI. Release and installed-binary verification remain pending; this file is progress evidence, not whole-task completion.
