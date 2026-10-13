# 交付与验证

R4 实现 `bb49708ea343f8e5cdf58e46217c7b1d48110b75`，0.2.25 版本提交 `f04b68329879e343770065d3a041963ac5bf0cf0`，Release run `38395342305` 成功。本地 cargo fmt/check/test 通过；全量 unit 217 passed/1 ignored，CLI contract 15、runtime control 13，其他 integration/doc tests 通过。

CI `38395342367` 唯一失败为 cargo-about 生成清单的自身版本仍为 0.2.20。使用 CI 同版本 cargo-about 0.9.0 的原生 generate 更新为 0.2.25，结果只有一行内容差异；无依赖或运行代码变化，无需再发布。

本机原生 lifecycle `upgrade --component fastctx --yes` 已返回 changed；最终版本/就绪核验由根任务记录。未删除真实 job 历史，未终止活动作业。既有 `10-11-codex-native-nesting` 未提交任务保持原状。

追加验收发现 0.2.25 在创建新 reservation 前回收历史，仅按活动计数判断准入，可能保留过多终态数据或低估旧大作业。0.2.26 改为先登记 pending 再回收，在同一 admission 锁内按实际字节、活动预留/实际使用较大值和终态/未知占用核验；统计错误传播，不能归零。跨会话四作业/第五拒绝/终态替换回归，以及 pending 回收和旧大作业保护两项单元回归均通过。

追加实现全量 cargo test 通过：unit 219 passed/1 ignored、CLI 15、runtime control 14、其余 integration/doc tests 通过；另有平台专属 parent-watch 1 ignored。最终 0.2.26 metadata 下 cargo check、fmt、clippy --locked --all-targets --all-features -D warnings 通过，许可清单由原生生成器刷新。最终发布/安装回执继续由根任务记录。
