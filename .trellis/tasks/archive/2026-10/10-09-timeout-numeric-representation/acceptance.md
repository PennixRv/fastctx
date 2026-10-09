# Timeout representation reconciliation

Root coordination: `codex-workflow-optimization/.trellis/tasks/10-09-workflow-footprint-reconciliation`.

- Fixed the shared applied-state checker, preserving exact timeout values and all other receipt/ownership checks. Integer and equivalent float TOML values now match; fractional changes, strings, `nan` and `inf` remain drift.
- `cargo test --locked --test cli_contract`: all 10 tests passed. The regression runs the real isolated apply/status/unapply route with seven positive/negative configurations.
- `cargo fmt --check` and `git diff --check`: passed.
- `cargo test --locked --lib control::`: all 9 shared control tests passed; workflow provenance verified at the explicit immutable commit.
- The initial `v0.2.22` release matrix built all platform artifacts, but the final distribution guard falsely rejected the tracked lowercase Trellis `agents.md`; no release artifacts were published. The pushed tag remains immutable.
- Retargeted the coordinated release to `0.2.23`, fixing the guard with case-sensitive private-path checks. Cargo package/lock and every npm platform/alias manifest are aligned; allocator-api2 remains `0.2.21`.
- Native project update moved generated Skills to beta.41. Explicit native workflow refresh adopted the same reviewed Marketplace commit as the root (`85d17acef228127cabe62bc409a16b6cf24f1d29`); preserved project-specific assets.
- `cargo fmt --check` and `cargo test --locked --test cli_contract` pass (10 tests). Local PowerShell is unavailable; release CI supplied the PowerShell distribution and exact npm tarball/install contract proof.
- First `0.2.23` CI run passed four platform jobs; Windows x64 timed out one existing MCP contract test after 10 seconds (`jobs_kill_manages_a_persistent_job_and_is_idempotent`), unrelated to the changed distribution guard. Failed-job retry passed unchanged; no test or release gate was relaxed.
- CI `37870858757` succeeded across all five platforms and the final distribution, annotated identity, npm install/publication, and GitHub Release gates. Release `v0.2.23` points to `5a67b3c203ffe0d0d8cc497d6e0b07d962b783b3`; `v0.2.22` was not rewritten.
- Pennix catalog and complete installed collection now consume `0.2.23`. Native lifecycle upgrade and Apply refreshed `/usr/bin/fastctx` and `~/.fastctx/bin/fastctx`, both reporting `0.2.23`; status is exit 0, applied state/binary/nine-tool hashes pass. Full lifecycle verification has no failures. Current host process hot reload is not claimed.
- Final project native workflow verify and update dry-run pass at beta.41; custom source matches the root. Retired backups/candidates and staging are absent, source main is pushed. All task acceptance criteria are met; native archive and journal close the records.
