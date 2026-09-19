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
