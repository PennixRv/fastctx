# 交付与验证

R4 实现 `bb49708ea343f8e5cdf58e46217c7b1d48110b75`，0.2.25 版本提交 `f04b68329879e343770065d3a041963ac5bf0cf0`，Release run `38395342305` 成功。本地 cargo fmt/check/test 通过；全量 unit 217 passed/1 ignored，CLI contract 15、runtime control 13，其他 integration/doc tests 通过。

CI `38395342367` 唯一失败为 cargo-about 生成清单的自身版本仍为 0.2.20。使用 CI 同版本 cargo-about 0.9.0 的原生 generate 更新为 0.2.25，结果只有一行内容差异；无依赖或运行代码变化，无需再发布。

本机原生 lifecycle `upgrade --component fastctx --yes` 已返回 changed；最终版本/就绪核验由根任务记录。未删除真实 job 历史，未终止活动作业。既有 `10-11-codex-native-nesting` 未提交任务保持原状。
