# 修复 Release 合同与版本发布回归

## Goal

修复 scoped npm 发布合同检查、动态 release notes 和历史 artifact 回归覆盖，确保发布失败时停在 npm publish 之前且不会因跨文件检查遗漏重复失败。

## Requirements

- `verify-distribution-contract.ps1` 必须分别读取并检查 `verify-npm-install.ps1` 与 `verify-launcher-lifecycle.js` 的合同。
- alias npm fixture 必须按 scoped forwarding launcher 解析真实主 launcher，不能把 alias launcher 当作主 launcher 的 fallback fixture。
- release notes 必须使用当前 annotated tag 的版本，不得固定写入某个历史版本。
- 保留 artifact 目录隔离、scoped npm launcher 路径、scoped provenance、alias 依赖和五平台产物合同。
- 不修改 Rust runtime、MCP 行为、Apply/TUI 行为或用户 `AGENTS.md` 行为。

## Acceptance Criteria

- [ ] PowerShell distribution contract 在 CI runner 通过，且错误合同归属有回归覆盖。
- [ ] release notes 与触发 tag 版本一致，静态检查不存在固定历史版本标题。
- [ ] `cargo test --locked --lib`、Node 语法检查和可用的 PowerShell 检查通过。
- [ ] 新 annotated tag 的五平台 build、artifact finalization、release verification、npm fixture install、publish 和 GitHub Release 全部通过。
- [ ] 七个 scoped npm 包与 GitHub Release 资产均可查询到同一版本。

## Out of Scope

- 不引入新的发布系统、版本清单或 artifact ledger。
- 不修改工作流之外的 runtime 源码和用户级配置。
