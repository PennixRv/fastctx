# 记录运行作业资源预算交付

## Goal

承接根任务 revision 2 的 FastCtx R4；记录已发布实现与本地测试，刷新原生生成许可清单并收敛 CI

## Requirements

- 记录已批准根任务 R4 的 1 GiB 总预算、256 MiB running reservation、跨进程 admission、终态回收与 drain/no-persist 交付。
- 许可清单已刷新；验收复核补齐新 pending reservation 纳入终态回收的时点、旧大作业实际占用保护和准入错误回执，不增加依赖。

## Acceptance Criteria

- [ ] 追加修复发布 0.2.26，本地完整 cargo test 通过且发布成功可核验；0.2.25 仅为中间版本。
- [ ] 新预留先计入账本再回收，实际占用/活动 charge/终态占用超预算时拒绝准入，未知状态不当作空闲；运行作业保留。
- [ ] 许可清单与 Cargo.lock 一致，CI 失败原因处理，原生 lifecycle 安装/核验结果回写根任务。

## Notes

- 授权继承根任务 revision 2 已批准范围。此前实施未及时建立组件任务，当前补齐落点，不虚构早期状态。
- 运行实现 `bb49708`、发布 `f04b683`；回滚到 0.2.24 需走 lifecycle 固定来源，不删除 job 历史。
