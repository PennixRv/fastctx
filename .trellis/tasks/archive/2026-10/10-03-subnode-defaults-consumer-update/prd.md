# Adopt published Trellis subnode defaults

## Goal and authorization

The user authorized commit, release and full adoption on 2026-10-03. This owner task applies published Trellis 0.7.0-beta.24 assets to `/home/penn/devel/fastctx`; root coordination remains in subnode-provider-config-diagnosis.

## Requirements

- Use installed native Trellis dry-run and create-new update paths; review each candidate before accepting or retaining a customization.
- Select codex-subnode-channel explicitly from gh:PennixRv/marketplace at 8f7a3741a107288ffe30a6c6ccc413f68d470c97; preview, materialize and verify native provenance. Preserve the pre-existing release.yml edit and unrelated untracked project assets.
- Adopt the eight shipped Sol/Luna profiles without default code_path; keep support for arbitrary project profile IDs.
- Preserve project specs, task/workspace state, private config and unrelated work. Do not change production component code or upgrade other components.

## Acceptance Criteria

- [x] Native project version is beta.24; template/hashes and generated profile bytes are verified against the published package.
- [x] Intended workflow selection and existing execution constraints are verified; each update candidate has a recorded disposition.
- [x] Scoped changes and publication state are recorded without swallowing pre-existing edits.

## Validation and recovery

Run native dry-run before/after, check the profile payload and git diff, and record retained project customizations. Git-tracked prior bytes and native candidates provide rollback; avoid directory-wide force or hand-edited hashes/provenance. Keep this lightweight task PRD-only until a material contract decision requires a replan.

## Generated asset commit boundary

For durable full adoption, commit the exact native receipt-listed generated files (all current bytes independently match), plus native metadata and this task. Exclude pre-existing specs, bootstrap task, old research, workspace, release.yml and retired local env. This does not publish or repair the FastCtx component itself.
