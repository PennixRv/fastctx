# beta.26 consumer check

2026-10-03. Native consumer update passed.

- `trellis --version` and `.trellis/.version` both report `0.7.0-beta.26`; workflow provenance verifies against the unchanged immutable Marketplace ref `1e0af97b9148ba8104f28577349af41c0b41868c`.
- Native dry-run and `trellis update --skip-all` refreshed the three bundled Channel references and version/hash receipt. Existing modified `.trellis/agents/subnode.env` was reported as skipped; the task-owned deleted root `.trellis/agents/subnode.env` remains unchanged. Second dry-run reports no pending template update; worker guidance contains `--workers`.
- The three reference updates and native receipts were committed and pushed to owner main as `4698a3a`. The existing `.github/workflows/release.yml`, `.trellis/spec/`, other tasks, and workspace remain outside the commit.

The task records are archived directly on owner `main`; main/base are self-referential because this bounded consumer task was authorized for direct main delivery and was never PR-backed. Use native `archive --skip-branch-validation` for that documented exception.
