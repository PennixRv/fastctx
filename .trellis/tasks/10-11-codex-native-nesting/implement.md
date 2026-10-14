# Implementation Plan

## Preconditions

本 task 与根/Pennix/Windsurf 的当前 native seal revision 一起展示；后续完整修订包明确批准才 native approve/start。主会话 inline，源编辑前 trellis-before-dev；现在不写产品或测试。

## Steps

本任务已于 2026-10-14 回到 planning。以下新增范围优先于旧步骤执行；旧 direct-only、配置事务、原生嵌套和发布步骤继续适用，但必须与 Revision 2 设计和根任务当前封口版本一致。

0. 先补本 revision 的 PRD/design/implement、实际 caller/规范和回归矩阵；完成根任务所需的 planning seal 后，等待本任务当前修订的后续实施批准。
1. 以现有 `guarded_burst_contract`、`server_contract`、`runtime_control_center` 和 output/job 单测固定共享池、临时 allowance、单次预算覆盖、交付游标及参数前置解析的真实基线。
2. 删除轮次累计池和旧 provider 的固定单次覆盖，保留每次响应的有界默认、显式配置、最低响应空间、取消和真实错误边界；修正容量提示与错误分类。
3. 修正 `server_support` 与 `job_output` 的交付顺序，使 withheld/缩页结果不推进未交付 cursor；确认 `run`、background、kill、replace 的副作用不因恢复逻辑重跑。
4. 保持规范参数唯一权威；追踪并修正调用域、schema、模型说明和 Pennix Skill 中的误用来源，改善 rmcp 前置错误诊断，不增加别名或通用参数猜测。
5. 在上述回归通过后再执行旧步骤中的配置/嵌套、文档、发行、安装和实际宿主重建门；任何 host contract 或输出保护变化回根任务 replan。

9. 记录净化验证/来源/真实限制到本任务及根矩阵；逐 AC 完成后原生归档、session 和源收敛提交推送。安装消费提交与纯收敛记录提交区分。

## Validation Matrix

| 门 | 样本 | 关闭条件 |
| --- | --- | --- |
| 配置/归属 | fresh、有效旧 0.2.23、已新 policy、无 receipt、错 profile/权限/符号链接/未知版本/漂移 | 无强制 direct-only；只清理已证明旧 owned 元素，其他值/注释保留；无 proof 不改 |
| 幂等/逆操作 | repeated apply、新版卸载、用户后来添加自己的 namespace | 不重复添加或删除用户项；MCP/settings/receipt 其他合同仍正确 |
| 可选指导 | 当前 v4 exact receipt、未知/手改 marker、默认 none、explicit guidance 接纳、新 v5 | default 不写；仅原生 explicit 可升级有效旧字节，新模型说明/段一致，未知不改 |
| 故障 | snapshot 并发、TOML 错误、配置/receipt/二进制写入故障 | 拒绝或精确事务回退，无半接入状态 |
| 输出/宿主 | 真 MCP 有限多调用、inspect files、Unicode 长行/Partial 与后续 offset | 直接与嵌套能力真实存在；内容、页码、错误不丢，缺失即 blocked |
| 源/发布 | 现有 CI/五平台 release/package/identity/SHA256SUMS | 固定提交和 registry/artifact 一致；已有版本不覆盖 |
| 组合 | 根 44 场景的 FastCtx 单体与 C01、当前宿主反例 | 普通本地不走 shell 包装；协议操作保留 native owner；真实动作证据可审计 |

## Evidence And Recovery

研究、失败和实际结果写本任务 research，根只保留引用/净化结论/来源，原始日志/配置/receipt/凭据不入 Git。任一有界单元结束立即记录再推进。变更来源、发布合同、输出机制、实验开关或验收不足需要根/源同步 replan；回退遵循 design，不覆盖用户新改动或既有公开版本。
