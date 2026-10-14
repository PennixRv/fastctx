# 修复实施与质量门记录

执行日期：2026-10-14。本文承接已封口计划，记录实际修改和验证结果；不改写前述规划事实。

## 实施范围

- 删除 `context_guard` 的每连接累计 burst 池、stub 和“下一轮”交付路径，保留每次响应自己的预算、分页和后台安全重试。
- 移除 Guarded 对宿主 10000、FastCtx 9000 及固定共享池的强制覆盖，使其使用选定档位的 host limit、FastCtx budget 和工具预算。
- 删除 TUI 对 Guarded 档位的锁定及旧固定值提示，更新多语言兼容性文案。
- 在 read、grep、run 和 run_background 的 rmcp 参数反序列化边界提供规范字段诊断；不增加 `path`、`query` 或 `max_output_tokens` 别名。
- 更新模型可见的参数说明与回归合同；新增同一连接连续大响应和共享池移除的测试。
- 版本从已发布的 `0.2.26` 提升为待发布的 `0.2.27`；没有覆盖已有 tag 或 npm 版本。

## 验证结果

- `cargo fmt --all`：通过。
- `cargo clippy --locked --all-targets -- -D warnings`：通过。
- `cargo test --locked`：通过；216 个库测试通过、1 个既有测试 ignored，全部 CLI、兼容性、glob、Guarded burst、URI、父进程、PDF、property、runtime control center、server、shell 和 doc test 通过。
- Pennix `skills/pennix-fastctx-routing` 合同测试：6/6 通过。
- `git diff --check`：FastCtx 与 Pennix 均通过。
- 版本核对：远端 `v0.2.26`、`@pennixrv/fastctx@0.2.26` 和 `@pennixrv/codex-fastctx@0.2.26` 均已存在；本修复目标为 `0.2.27`。

## 发现与处置

全量测试暴露两处旧合同：guidance-v4 测试依赖生成块的冻结字节，曾因一次不必要的模型说明改动而替换未命中；已撤回该生成块变更并恢复原有冻结合同。runtime Guarded 测试仍要求固定 9000 上限和下一连接释放；已改为验证选定档位的单次边界、同一连接连续调用和可识别终态。两处均为本任务范围内的机械合同修正。

## 尚待完成

下一步按现有五平台 Release workflow 提交并推送 `0.2.27`，核验 CI、GitHub Release 和七个 scoped npm 包；随后由 Pennix lifecycle 更新 `approved_version`，重装 FastCtx 与路由 Skill，完成新进程/新 MCP 连接及根仓库集成验收。
