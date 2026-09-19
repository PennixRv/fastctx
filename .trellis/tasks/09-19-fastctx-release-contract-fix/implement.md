# Release 合同修复实施计划

1. Patch the contract reader and dynamic release notes.
2. Add focused regression coverage without duplicating the release pipeline.
3. Run Node syntax, Rust library tests, and PowerShell contract checks where available.
4. Commit and push the source branch, create the required annotated version tag, and trigger the existing release workflow.
5. Verify all build jobs, publish job, five release archives, `SHA256SUMS`, seven npm packages, and GitHub Release metadata.

## Stop Conditions

- Contract or artifact verification failure: stop before publish.
- Missing or mismatched npm/package/release evidence: do not update bootstrap catalog or reinstall.
- Runtime behavior change required: split a separate task.

## Execution Record

- Source commit: `d81dcde` (`fix: close release contract propagation gap`)
- Annotated tag: `v0.2.12`
- Actions run: `35418742229`
- Local checks: seven npm manifests parsed, Node syntax check passed, `cargo test --locked --lib` passed (`214 passed; 1 ignored`)
- All five build jobs and contract checks passed; publish stopped in the alias fallback fixture because only the retired unscoped forwarding string was recognized.
- Windows PowerShell contract passed in CI; external publication remains blocked until the alias fixture fix is released.
- `v0.2.13` reproduced the same fallback failure because the patch landed in the first duplicate parser rather than the fallback branch; the fix now centralizes all three parser call sites.
