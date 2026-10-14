# FastCtx Codex 原生嵌套接入与输出完整性

## Goal

承接根任务 N02/D11/D12 以及根任务 `fastctx-output-pool-recovery`，取消 mcp__fastctx 强制 direct-only，修复按“用户轮次”累计的共享输出池和单次预算强制覆盖，保留直接调用和其他 owner 原生边界；同时核验输出交付游标、真实错误分类和参数误用根因，规划受管配置升级回退、实际宿主暴露、聚合输出和续页完整性门，批准后实施并完成必要发布重装。

## Requirements

- R1：落实根 N02/D11/D12，取消 mcp__fastctx 的强制 direct-only 接入。保留直接调用能力，允许宿主按实际能力原生嵌套同一 MCP 工具；不新增只许嵌套限制，不自动启用 Code Mode 等实验开关。
- R2：普通本地 read/search/CLI/job 的操作范围保持，Trellis 状态、交接、原生交互、外部检索和部署的协议排除由其原生 owner 维护，不扩展 FastCtx 的职责。
- R3：源码修改覆盖 codex_config apply/unapply/drift、apply preview/receipt、doctor、共享模型说明及可选 guidance 的真实消费者、直接调用者与 golden/CLI 测试、当前 README 接入说明。可选 guidance 只按原生 explicit 接纳，默认 none 不写 AGENTS。复用现有 TOML/事务/输出 guard，不新增包装器、路由器、Hook 或依赖。
- R4：从已核验的受管 0.2.23 配置升级，仅去掉属于旧接入合同的 mcp__fastctx 元素。未知/无 receipt/漂移配置不得接管，其他 namespace、注释、用户配置、权限和不相关字节保留；失败按原有事务回退。
- R5：聚合输出和 Partial 续页是交付门。覆盖新装、旧受管升级、新版本幂等/卸载、无归属/漂移拒绝、并发与写入故障，以及实际宿主中直接与原生嵌套调用；不能只测配置或声明工具已暴露。
- R6：与 Pennix 的共享数组解除钉死、catalog 和路由内容同步交付。原生接口更新后实际宿主须重建工具清单；缺失/聚合截断/页码丢失保持 blocked 并返回规划，不增加 developer 指令弥补。
- R7：源 main 基线 8661fd8b52588777917bc3d526063501e5737ceb，拟发布 0.2.24。使用现有五平台 GitHub Release 与 scoped npm 工作流；发布前重查占用，不覆盖已发布 tag/version。最终固定来源后由 lifecycle 原生升级与 apply。
- R8：只在本任务当前 seal 展示后的完整修订包明确批准、原生 approve/start 后实施；当前只写计划。模型门沿根 D6，44 场景前后和两类交互，总轮次至多 100，本机串行。
- R9：取消按调用间隔猜测用户轮次的累计共享输出池，保留每次响应的有界预算和分页；容量不足不得要求新用户消息或把临时额度报成固定环境变量配置错误。
- R10：取消 Guarded 对单次工具预算的强制 9,000/10,000 覆盖，尊重显式配置与宿主实际单次上限，保留有界默认、执行前最低预算和真实错误保护。
- R11：不为 `path`/`file_path`/`query` 增加兼容别名；追踪调用域、工具 owner、发布 schema、模型/Skill 说明和参数构造处的误用根因，规范字段保持唯一权威。
- R12：输出交付和游标推进必须一致；未交付结果不得推进可恢复游标，已执行命令不得通过恢复逻辑盲目重跑。

## Acceptance Criteria

- [ ] AC1：apply/doctor/receipt 不再把自己的 namespace 必须 direct-only 作为健康要求，直接调用仍可用；新装不创建该限制。
- [ ] AC2：有效旧 0.2.23 的受管限制精确移除，未知/用户漂移不覆盖，其他 namespace 与配置保真；幂等、卸载、并发和失败回退通过。
- [ ] AC3：实际重建宿主暴露原生嵌套 MCP，普通本地搜索不为批量改成 exec_command 或 Python/rg 包装；其他 owner 协议不走 FastCtx。
- [ ] AC4：有限并行 grep/glob、批量 inspect 与单体续页的实际轨迹完整，聚合与 Partial 不丢内容/页码/错误类别；受限项如实 blocked。
- [ ] AC5：现有必要 CI/发行矩阵通过，来源/五平台摘要/scoped npm 版本一致，0.2.24 推送和原生安装/apply/现场核验完成并回写根任务。
- [ ] AC6：同一 MCP 连接连续分页无需新用户轮次即可推进；临时额度不足不再产生固定环境变量配置错误或普通工作的终止提示。
- [ ] AC7：显式单次预算不被旧 provider 的固定 9,000/10,000 覆盖，仍有单次有界输出、最低响应空间和真实失败保护。
- [ ] AC8：规范参数错误能够定位工具、字段和纠正方法；不接受别名，不进入实际副作用处理，误用根因已在调用路径或说明 owner 修复。
- [ ] AC9：`job_output`、前台命令和各分页工具的交付范围、Complete/Partial 状态和 cursor 一致；已有副作用不会被重跑。

## Notes

- 根协调任务：/home/penn/devel/codex-workflow-optimization/.trellis/tasks/10-11-pennix-skills-audit-gap-remediation/；真实决策和因果证据为其 decisions.md D11/D12 与 research/07-fastctx-routing-incident.md。
- 初始工作树 main 干净，原生 current 无活动 task 且有真实 session_source；本任务由 create --no-start 建立，当前 planning。无子代理，未改产品、配置或运行版本。
- 本任务于 2026-10-14 经原生 `task.py replan` 从未收敛的 in_progress 状态返回 planning；根任务已决定取消轮次池、取消单次强制覆盖、拒绝参数兼容，并要求修复调用根因。当前只更新任务资产，未启动实施。
