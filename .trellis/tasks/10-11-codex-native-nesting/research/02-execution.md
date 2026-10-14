# 修复实施与质量门记录

执行日期：2026-10-14。本文承接已封口计划，记录实际修改和验证结果；不改写前述规划事实。

## 实施范围

- 删除 `context_guard` 的每连接累计 burst 池、stub 和“下一轮”交付路径，保留每次响应自己的预算、分页和后台安全重试。
- 移除 Guarded 对宿主 10000、FastCtx 9000 及固定共享池的强制覆盖，使其使用选定档位的 host limit、FastCtx budget 和工具预算。
- 删除 TUI 对 Guarded 档位的锁定及旧固定值提示，更新多语言兼容性文案。
- 在 read、grep、run 和 run_background 的 rmcp 参数反序列化边界提供规范字段诊断；不增加 `path`、`query` 或 `max_output_tokens` 别名。
- 更新模型可见的参数说明与回归合同；新增同一连接连续大响应和共享池移除的测试。
- 最终发布目标从已发布的 `0.2.26` 提升为 `0.2.29`；没有覆盖已有 tag 或 npm 版本。

## 验证结果

- `cargo fmt --all`：通过。
- `cargo clippy --locked --all-targets -- -D warnings`：通过。
- `cargo test --locked`：通过；216 个库测试通过、1 个既有测试 ignored，全部 CLI、兼容性、glob、Guarded burst、URI、父进程、PDF、property、runtime control center、server、shell 和 doc test 通过。
- Pennix `skills/pennix-fastctx-routing` 合同测试：6/6 通过。
- `git diff --check`：FastCtx 与 Pennix 均通过。
- 版本核对：远端 `v0.2.26`、`@pennixrv/fastctx@0.2.26` 和 `@pennixrv/codex-fastctx@0.2.26` 均已存在；最终修复版本为 `0.2.29`。

## 发现与处置

全量测试暴露两处旧合同：guidance-v4 测试依赖生成块的冻结字节，曾因一次不必要的模型说明改动而替换未命中；已撤回该生成块变更并恢复原有冻结合同。runtime Guarded 测试仍要求固定 9000 上限和下一连接释放；已改为验证选定档位的单次边界、同一连接连续调用和可识别终态。两处均为本任务范围内的机械合同修正。

## 发布进度

- 修复提交 `a5c56f7` 已推送。首次 `v0.2.27` 发布运行 `38574427951` 在 Stage npm platform package 阶段失败：七个 npm 清单仍为 `0.2.26`，与 Cargo/tag 的 `0.2.27` 不一致。
- 已统一 Cargo、lock 和七个 npm 清单为 `0.2.28`，提交 `117bb1375e87c19ad51caa4c5548073454c77184` 并推送。`v0.2.27` 失败 tag 保留，不改写；新发布运行为 https://github.com/PennixRv/fastctx/actions/runs/38577164882 。
- `v0.2.28` 五个平台构建全部成功，但 publish-release 的本地 npm 安装验证发现主包 optionalDependencies 和兼容包 dependencies 仍指向 `0.2.26`；尚未进入 npm 发布或 GitHub Release 创建。
- 已统一 `0.2.29` 的 Cargo、七个 npm 包及所有包内引用，并在既有 verify-distribution-contract.ps1 加入版本一致性检查。独立本地版本核验 7/7 通过；本机没有 pwsh，PowerShell 门由现有 CI 执行，不用等效静态核验冒充其执行结果。

## 重启后运行验收

2026-10-14 重启当前 Codex 会话后，当前 FastCtx MCP 服务进程由稳定入口 `/home/penn/.fastctx/bin/fastctx` 启动，进程和命令版本均为 `0.2.29`。当前会话直接调用 `grep`、`glob`、批量 `inspect_local_file` 和分页 `inspect_local_file` 均通过；服务返回的精确 `offset` 被用于续读并达到 `Complete`，没有依赖下一用户轮次。

新进程探针 `/tmp/fastctx-installed-runtime-check.json` 为 `ok=true`：同一连接并发大响应均完整返回，6000 行正文和 6001 行分页正文连续交付为 3 页，批量读取通过；规范参数成功，顶层/批量字段误用和 `run.max_output_tokens` 错误均在解析边界给出诊断，错误命令未执行。`fastctx status` 的应用配置、安装二进制、MCP contract 和 AGENTS guidance 均为 PASS；其中 model-side tool surface 仍是 FastCtx 无法自观测的 INFO，本次当前会话的直接 MCP 调用作为实际宿主入口证据。

最终发布 `v0.2.29`、五平台/registry/包引用一致性、Pennix 安装集合和根侧 8/8 离线集成已在根任务记录；当前源工作树 clean，`origin/main` 为 `9d655614a326bf543d81697a747bc230259e0c19`。旧会话进程的生命周期仍以其会话退出为准，不把历史进程与本次新连接混为一谈。
