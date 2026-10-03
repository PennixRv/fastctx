
## Workflow adoption decision

The old codex-subnode-channel workflow had no source provenance. Explicitly selected and previewed the same workflow from gh:PennixRv/marketplace#8f7a3741a107288ffe30a6c6ccc413f68d470c97. Preview bytes exactly match source SHA256 d6903f7eda75fca87994b0254579b0f0cc8e19bed924ac732d0c0aa7e508c963. Accept via native workflow materialization and verify; remove only this exact accepted preview afterward. Legacy subnode.env is preserved as pre-existing local state and is no longer referenced by the updated native subnode role.

## Published adoption checkpoint

- Project version is 0.7.0-beta.24; all eight profile bytes and the native receipt equal published source SHA256 b862bd997cb8874ae0e167ca47abcd315840e6a6102373c3e516e969f14ee8a2. Python scripts and present Codex hooks parse successfully.
- Native post-update dry-run has no remaining new/auto-update files. CCH/Windsurf retain exactly the five declared custom files and previously deleted native agent assets; other updated projects are already up to date. No directory-wide force, migration, private configuration, or hand-edited metadata was used.
- Root and FastCtx provenance verify codex-subnode-channel @ 8f7a3741a107288ffe30a6c6ccc413f68d470c97; Trellis and Skills explicitly verify native @ beta.24.
- Scope is released defaults and consumer adoption. Prior two read-only/no-tools route smokes establish Sol/high and Luna/xhigh reachability only; role quality, full Channel/report reliability and the three downstream tasks remain outside this acceptance.
- Consumer generated assets were previously untracked. This full adoption commits only native receipt-listed bytes (all 100 match), native version/hash/provenance metadata, the generated ignore file/journal merge rule and this task. Project specs, bootstrap task, old task research, workspace and release.yml WIP remain excluded. Retired local subnode.env is preserved and inert.
