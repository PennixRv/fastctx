# Design

## Scope And Decisions

根 D12 已明确允许 FastCtx 原生 MCP 嵌套，保留直接调用；不用增量 developer 指令协调，不替换内置提示，不修改二进制或缓存。实际配置策略和输出保护在本仓库改，Pennix 只修其静态模板与路由/catalog，不能在根仓复制组件实现。当前全部为计划。

## Owned Configuration And Upgrade

新 apply 不创建、不要求 mcp__fastctx 的 direct-only 元素，也不启用 code_mode/code_mode_host 或重写整个 namespace 数组。其他用户 namespace 继续由其 owner 管理。当前 codex_config::apply/drift_applied、doctor 必需项和 preview 的强制加号同步改变；不是只改 README。

唯一清理入口是原生 apply 的同一事务。从已知受管 0.2.23 接入合同升级时，先核对可信 receipt 的版本/目标 profile、安全路径、现行 FastCtx MCP 与其他实际受管字段和旧 namespace 元素的语义归属，再精确删除旧 mcp__fastctx 元素并保留数组其余项、尾注释和不相关字节。现有 ManagedFileRecord 明确以受管语义核验共享配置；不把其他 owner 的合法修改造成的整文件 hash 变化当作本 scope 漂移。receipt 不符、未知版本、实际受管字段漂移/并发/不安全对象或无归属时，不移除用户元素、不声称嵌套就绪；停止该依赖动作并保留具体冲突。

复用现有 AppliedRecord.version 和 managed-file proof 区分 0.2.23 的旧接入合同与新 0.2.24，避免为单个已知合同新建迁移 registry/通用策略配置。新 receipt 不再拥有 namespace 元素；新 unapply 不能删除用户后来选择的 direct-only 项。旧版已有安全逆操作和历史 server 清理继续按其 receipt 处理，本轮不扩大成任意旧安装迁移。共享文件随后由其他已授权 owner 修改时保留真实差异，不把旧整文件 hash 当作删除任意内容的许可。

apply 中前后配置、receipt、二进制和 settings 使用已有 transaction snapshot/compare/commit/rollback；删除空表只在确实没有其他键/注释需要保留时执行。预检查错误发生在写入前，任何后续故障恢复精确旧内容。首次存在但无本组件 ownership 的同名 namespace 保留，由根实际宿主门报告冲突。

## Caller And Artifact Map

| 维护点 | 当前 caller/关联资产 | 必需同步 |
| --- | --- | --- |
| src/control/codex_config.rs | apply.rs plan_apply/unapply、drift_applied、has_namespace 与单元测试 | 接入期望、严格 TOML/元素保真、旧 proof 清理与新 uninstall 分工 |
| src/control/apply.rs | plan_apply、AppliedRecord、preview details 与 transaction | 旧 proof 读取先于 namespace 改动；新 record/version、预览不再声称强制 direct-only、事务回退 |
| src/control/doctor.rs | 受管字段存在性与 drift_applied | 删除强制 namespace 的 missing/drift；保持 MCP/参数/输出/receipt 其他必需项 |
| src/model_guidance.rs、src/control/agents.rs | MCP namespace/inspect 描述、可选 guidance 的共享正向说明/合同与旧 exact section | 活跃文字去掉可能被读为“仅顶层”的限制；同一 native MCP 可按宿主编排，路径/URI 与工具选择保证保留。新 guidance-v5 只通过 explicit guidance 接纳，有效旧 v4 字节/receipt 精确升级；默认 apply --guidance none 不写 AGENTS |
| tests/cli_apply_golden.rs、tests/cli_contract.rs 及原模块单测 | apply/legacy fixture、preview/golden/卸载 | 对新装、精确受管升级、用户项/注释、无 proof 和回退验证最终行为 |
| README.md、README.zh-CN.md | 接入策略/修改对象、Pennix Skill 入口 | 说明直接/原生嵌套都可用，按实际宿主选；同步新 lifecycle Skill 名，不宣传全局 shell 强制路由 |
| Cargo.toml/Cargo.lock、packages/fastctx* 与 packages/codex-fastctx/package.json | 现有 scripts/verify-distribution-contract.ps1、发布 workflow | 主包、五平台与现有 codex-fastctx compatibility alias 的版本/依赖统一 0.2.24；按现有 scoped family 发行，不创造新 alias 或上游 npm 名称 |

实现前再次 grep 实际 caller，以上不是允许漏掉新发现的真实消费者；新实质 owner/权限/机制变化返回根规划。源码规范目前 backend quality 仍为模板，不能冒称已填规范；代码及检查以已有模块和实际 CI 为证据，不附带全仓 spec bootstrap。

当前 LOCAL_FILE_ROUTE_GUIDANCE 既用于 MCP 描述也用于可选指导段；改变活跃短句时要同步精确 section 合同，不能把旧字节匹配样本全局替换掉。旧 stamped guidance-v4 在未 explicit guidance 接纳前保持 apply-required，不能当作 user drift 自动刷新；explicit 接纳只在旧完整字节/receipt 验证后更新 v5，新段依现有受管 marker/并发/事务验证。不增 guidance 写入方，不从本机系统部署自动推导修改所有用户的指导段。本机 Pennix 仍维护自己用户 AGENTS 模板，正常升级使用 guidance none。

## Native Host And Output Gate

保留逐工具单次预算、Complete/Partial 合同及共享运行时安全约束，但删除按调用间隔猜测用户轮次的累计输出池和旧 provider 的固定 9,000/10,000 单次覆盖。可嵌套只改变宿主可调用域，不把文件搜索包进 shell，不由 FastCtx 转发其他协议，也不新增宿主执行包装器。

在新的真实宿主工具清单上核对直接工具及编排器实际 FastCtx 可见性；不借本机 CLI code_mode=false/true 或配置文本推断当前 App。用无秘密有限 fixture 完成 C01：多个独立 grep/glob 通过真正 MCP 并行；多文本 inspect 用现有 files；对 Partial 的后续页属于依赖动作，按 native offsets 顺序读取。单次直接调用保留，不强制聚合所有大结果。

逐结果保留匹配/文件、Complete/Partial、下一 offset、错误类型和观察顺序；Unicode、长行、跨页与不超过实际宿主单次输出上限的批次均需实际完整。聚合中截断不得被洗成 Complete；容量提示不得要求新的用户轮次；若宿主真实单次合同不足，记录具体合同缺口，不用模糊 burst 池或固定旧版本数字遮蔽。44 场景数量与 100 模型轮次不因 N02 自动增加，native 无模型诊断另行标记，不冒充行为验收。

## Revision 2: Capacity, Delivery And Parameter Root Cause

根任务已关闭三项决定：取消轮次池；取消 Guarded 对单次工具预算的强制 9,000/10,000 覆盖；不增加参数别名兼容。实现应以单次有界预算和准确分页为边界，区分显式配置、实际宿主上限、临时可用额度、最小响应空间和真实错误。

`server_support` 不再把临时 allowance 格式化成 `FASTCTX_*_TOKEN_BUDGET=<temporary>`，也不以 `next turn` 作为普通恢复合同。若响应被暂扣或缩页，交付范围和 cursor 必须原子一致；`job_output` 未交付页面不得推进隐式游标，已执行 `run` 不通过恢复逻辑重新执行。错误分类优先使用明确状态，不以多个字符串包含关系推断跨层语义。

参数维持各 `tools/list` 发布的规范字段。对 `inspect_local_file` 顶层 `path`、批量条目 `file_path` 和 `grep.query` 不增加兼容；要从调用域、FastCtx owner、发布 schema、模型/Skill 说明和实际参数构造点找出误用来源。`run.max_output_tokens` 继续明确拒绝，错误应说明工具、未知/缺失字段、规范字段和纠正动作，不自动猜测或重跑副作用。

本 revision 先补能复现 V01/V03/V04/V07 的服务端回归，再按现有 native nesting、配置事务和发行合同实施。若取消轮次池后发现当前宿主确有明确的跨调用总量保护合同，回根任务重开具体合同决定，不恢复服务端无法观测的用户轮次模型。

## Release And Consumption

main 基线 8661fd8b52588777917bc3d526063501e5737ceb、0.2.23。拟发布下一个稳定补丁 0.2.24，不迁移 1.0。规划时 remote v0.2.24 查询为空、GitHub release not found，npm 指定版本返回 E404；这些是当时观察，发布前必须重查五平台/主包、tag 与 release。

运行既有质量与发行身份门，审阅后提交推送 origin/main；annotated v0.2.24 指向验收提交并沿现有 Release workflow 产出五平台归档、SHA256SUMS、scoped npm family。不得覆盖 tag/version，不自动重跑已有 release 发布；中途失败先核实已发布状态再决定恢复。不要给 tag 指向的 commit 加 skip-ci 标记。根/Pennix catalog 最终固定真实版本/SHA/摘要后，使用 lifecycle 原生升级和 fastctx apply --guidance none；不手改 ~/.codex/config.toml。当前会话无法自行获得新工具清单时等待真实宿主重建，不用伪造 namespace 或执行身份通过验收。

## Recovery

本地源回退使用正常反向提交；安装按原生事务/旧有效 artifact 恢复。npm/tag 已发布不可覆盖，必要修正另发补丁并返回实质规划，不降成“未发布”。配置恢复只使用验证过的原 snapshot，不覆盖之后用户变更。新实验开关、依赖、Hook、通用指令注入、额外模型预算或治理机制都属于重新决策条件。
