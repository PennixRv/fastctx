# Consumer candidate disposition

2026-10-03. Native beta.25 update completed with skip-all. The existing
codex-subnode-channel workflow is retained; its native preview from immutable
Marketplace commit 1e0af97b9148ba8104f28577349af41c0b41868c changes only the
three unbound recovery breadcrumb bodies. The full candidate diff was accepted.
After native application, remove only the byte-identical workflow.md.new.
Pre-existing release.yml, spec/bootstrap/task research, and workspace changes
are outside this task's commit.
- Native workflow verify passed for the exact Marketplace commit.
- Both recovery scripts match published templates, continue/recovery guidance
  passed, all 100 existing managed receipt entries match, and diff whitespace
  checks passed. The accepted identical preview was removed.
