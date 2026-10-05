# Reconcile release credential and existing Trellis records

## Goal

Root SiYuan workflow closeout: restore the confirmed GitHub NPM_TOKEN publish contract, review and preserve existing untracked Trellis scaffold and historical research, verify and push scoped main commits without declaring bootstrap complete.

## Requirements and boundaries

- Source-owner follow-up to the root SiYuan workflow delivery. The user reaffirmed on 2026-10-05 that npm publishing uses the GitHub `NPM_TOKEN` secret, and previously authorized scoped commits/pushes and full closeout.
- Restore only the two deleted `env`/`NODE_AUTH_TOKEN` lines in `.github/workflows/release.yml`, preserving the accepted release contract and every other byte. Git cannot attribute this uncommitted deletion to a person; do not claim the user removed it.
- Review and commit the 13 existing untracked Trellis text/JSON assets: nine spec scaffold/guides, two bootstrap task files, one historical control-plane baseline, and the shared workspace index. Preserve their existing content and record scaffolding as incomplete; do not declare older tasks accepted or archive them.
- Work on `main`, with no runtime/code/package-version change, release tag, secret access, credential rotation, or npm publication. This task does not fill the bootstrap specs or redesign CI authentication.

## Acceptance Criteria

- [x] Restored release workflow is byte-identical to HEAD and retains `NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}`.
- [x] Existing records are inspected, JSON parses, local document links resolve, and staged differences contain only the declared nonsecret records.
- [x] Native task validation and `git diff --check` pass with unused context manifests explicitly skipped.
- [x] Scoped main commits are pushed, repository is clean, and root acceptance records the resulting source commit.
- [x] This owner follow-up is archived correctly without changing older task states.

## Planning Seal

Closed 2026-10-05: lightweight preservation/restoration under existing delivery authorization and the user's explicit credential clarification. Main session performs the work; no worker or new implementation choice. Restore the exact accepted workflow; retain reviewed initial scaffolding and historical records without extending their scope. Validation is content/ownership integrity, not a new release or Rust runtime test.

## Verification

2026-10-05: all 13 preexisting records were inspected; the one JSON task record parses and nine local Markdown links resolve. The backend documents remain initial scaffolds and bootstrap remains `in_progress`; old task states are preserved. No private-key/npm/GitHub credential pattern was found in these records. Native task validation passed with unused implement/check manifests skipped. The restored workflow has no diff against HEAD, so this closeout changes no publishing behavior or runtime and requires no new package release or installation.

Source record commit `96b5c9e` was pushed to origin/main and the native finish survey reported a clean tree. Root research/11 records that fixed point. Native task archive succeeded with the main/non-PR branch-validation exception and no implicit commit; its move and both old tracked-path deletions are committed together. Bootstrap and the two older delivery tasks remain unchanged.
