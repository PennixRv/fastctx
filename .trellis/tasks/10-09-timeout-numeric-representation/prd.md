# Accept equivalent numeric MCP timeout representations

## Goal

Fix receipt drift false positives for exact equivalent TOML float timeouts and release filename-case validation, publish 0.2.23 without rewriting v0.2.22, update local project assets and verify native delivery.

## Requirements

- Accept exact integer/float timeout equality in the shared drift checker; preserve changed/nonfinite/nonnumeric rejection and legacy receipt behavior. Make the distribution-source filename check case-sensitive so the legitimate lowercase Trellis `agents.md` does not block release; retain the exact npm package allowlist. Release npm/GitHub 0.2.23, leaving pushed tag v0.2.22 unchanged; update beta.41 project assets/custom source and clean retired backups.
- Authorized by the user's explicit full reconciliation request; root plan: /home/penn/devel/codex-workflow-optimization/.trellis/tasks/10-09-workflow-footprint-reconciliation.

## Acceptance Criteria

- [ ] CLI regression and existing contracts pass; lowercase/uppercase distribution-path behavior and exact npm tarball contents pass; 0.2.23 release/install/native status and project provenance pass; source/consumer Git pushed and temporary backups removed.

## Notes

- Keep `prd.md` focused on requirements, constraints, and acceptance criteria.
- Lightweight tasks can remain PRD-only.
- For complex tasks, add `design.md` for technical design and `implement.md` for execution planning before `task.py start`.
