# 接入实现与验证现场

根、Pennix、Windsurf 与本 task 的 revision 1 已获得完整修订包明确批准并通过原生 approve/start。本记录承接已封口计划；prd/design/implement 保持原字节。当前源版本目标为 0.2.24，尚未发行或安装。

## 已实现的有界单元

- 新 Apply 不创建、追加或要求自己的 direct-only namespace；drift 与 doctor 删除这个健康前提，preview 按实际前后差异报告移除。新 unapply 不删除用户自行选择的同名元素。
- 唯一已知旧合同是 receipt.version=0.2.23。迁移前核对真实目标 profile、受管 command/参数/timeout/预算/共享限额、安装二进制摘要、元素唯一性和 receipt 安全路径。其他 owner 对共享 TOML 的合法修改不受整文件摘要限制。无 receipt 或未知版本不接管 namespace；旧 proof 冲突在任何写入前拒绝。仍用原 transaction 的 snapshot/compare/commit/rollback，不增加迁移框架。
- 原数组编辑保留 Unicode 注释、其他元素和空数组中的注释；没有为了删元素清空 features 表。默认接入继续不写 AGENTS。共享模型文字允许宿主真实暴露的原生编排与直接调用，要求逐结果/续页完整；截断先以有界直接调用补回。
- guidance-v5 保留 v4 两种 shell 状态的精确比较样本，并以固定摘要约束旧字节。具备有效 v4 receipt 的完整旧段只按显式 guidance 接纳升级；旧段/stamp 不匹配拒绝。默认 none 保留 v4，doctor 报告 Apply required。
- Cargo 与七个既有 scoped npm manifest 同步 0.2.24，README 同步真实配置归属和新的 Pennix lifecycle 名称。既有 npm compatibility alias 保留，不增加 Skill 旧名 alias。

## 已观察的结果

新装 golden 在改产品前复现 2 项失败：会创建 features 表、会追加 mcp__fastctx；原有其余 2 项通过。首次编译发现移除注释处理的 Rust 可变借用冲突，已读取完整错误并将拼接值先计算再赋值。后续同组 golden 4/4 通过，CLI 合同组仍在运行；其运行后二次加入的 v4/安全路径测试尚未执行，不计为通过。

本机为 Arch Rust 1.99.0，没有 rustup 或 pwsh。Rust 1.99 对既有 fetch_update 发出 deprecated 警告，而新 try_update 不满足声明的 Rust 1.88 最低版本；在原调用的 let 声明添加单点 allow(deprecated) 以保留 MSRV。该必要质量调整未改变算法，严格 clippy 与 MSRV/跨平台门仍待实际运行。

## 尚未完成

完整 Rust/CLI/guidance/安全对象/失败回退门已在本机通过：all-features 串行测试包含 217 个库测试、4 个 golden、15 个 CLI contract、1 个 v0.1.1 compatibility、1 个 glob、1 个 guarded burst、1 个 URI、6 个 property、13 个 runtime control center、8 个 server contract、11 个 shell contract、3 个 PDF 及其余 doc tests，既有 1 个 ignored 测试保持 ignored。此结果尚不包含 clippy、声明 MSRV/跨平台构建、发行身份或真实宿主暴露。

Pennix catalog/受管命令新增 Windsurf 映射后发现并修复了一个事务缺口：原子替换时已有受管链接必须重建，否则会继续指向被移入 backup 的旧树；旧 Grok 目标只有在 schema 2 receipt、旧命令 map、整树 digest 和旧 target 同时匹配时才可重定向。未知同名路径仍拒绝接管。FastCtx 本源代码尚未提交、发布或安装。

真实宿主暴露和聚合/Partial 门必须在原生受管重装后用真实新会话核验；当前 MCP 清单不是新版证据。根 44 场景的 before 基线仍使用旧安装，源代码修改不冒充后基线。下一步完成 clippy/MSRV/跨平台或其受限证据、源提交与不可覆盖的 0.2.24 发行，再推进 catalog 固定和原生重装。
