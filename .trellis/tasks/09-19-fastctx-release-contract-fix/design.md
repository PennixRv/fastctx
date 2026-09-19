# Release 合同与版本发布修复设计

## Ownership

只修改发布 workflow、PowerShell 合同脚本和必要的测试/文档。Rust runtime 保持不变。

## Contract Flow

```text
build-* → dist/release
npm-*   → dist/npm
  ↓
verify-distribution-contract.ps1
  ├─ verify-npm-install.ps1: install paths and package graph
  └─ verify-launcher-lifecycle.js: runtime provenance and lifecycle
  ↓
finalize SHA256SUMS → verify archives → pack root packages → fixture install → publish
```

The current blocker is a cross-file ownership error: the distribution contract reads the npm installer script but checks for a JavaScript provenance marker. Keep one content variable per owner and assert each marker against the correct file.

The first new-tag verification also exposed a fixture ownership error: the lifecycle fallback recognized only the retired unscoped forwarding string, so the scoped alias launcher was copied into the main-package fixture and failed to resolve its scoped dependency.

## Minimal Changes

1. Add a separate launcher verifier content variable and move the provenance marker assertion to it.
2. Derive release note title from `GITHUB_REF_NAME` rather than a fixed version literal.
3. Keep existing artifact, package path, alias and identity checks; add only focused regression assertions for the propagation errors.
4. Resolve both scoped and legacy forwarding launcher forms before copying the main launcher into the fallback fixture.

## Safety

- Preserve all scoped package names and artifact names.
- Preserve `NPM_TOKEN` as the only npm publish credential.
- Any verification failure continues to stop the workflow before publish.
