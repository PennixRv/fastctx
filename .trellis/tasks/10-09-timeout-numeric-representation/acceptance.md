# Timeout representation reconciliation

Root coordination: `codex-workflow-optimization/.trellis/tasks/10-09-workflow-footprint-reconciliation`.

- Fixed the shared applied-state checker, preserving exact timeout values and all other receipt/ownership checks. Integer and equivalent float TOML values now match; fractional changes, strings, `nan` and `inf` remain drift.
- `cargo test --locked --test cli_contract`: all 10 tests passed. The regression runs the real isolated apply/status/unapply route with seven positive/negative configurations.
- `cargo fmt --check` and `git diff --check`: passed.
- `cargo test --locked --lib control::`: all 9 shared control tests passed; workflow provenance verified at the explicit immutable commit.
- Version bumped together in Cargo package/lock and every npm platform/alias manifest to `0.2.22`; no dependency update.
- Native project update moved generated Skills to beta.41. Explicit native workflow refresh adopted the same reviewed Marketplace commit as the root (`85d17acef228127cabe62bc409a16b6cf24f1d29`); preserved project-specific assets.
- Release and installed-binary verification remain pending until the tag workflow completes. This file is progress evidence, not whole-task completion.
