dtv-core迁移规范（Phase1）

本文档定义第一阶段（解耦）迁移的强约束、目录边界、编码规范与验收规则。目标是：在保持前端协议不变的前提下，把可复用业务逻辑集中到 `dtv-core`，并确保 `dtv-core`完全不依赖 `tauri`。

1.命名与目录约定

-Workspace结构（目标）：

-`Cargo.toml`（workspaceroot）

-`src-tauri/`：Tauri壳层

-`dtv-core/`：纯Rust业务库（cratename:`dtv-core`,crateid:`dtv_core`）

-Rust命名：

-crate目录使用中划线：`dtv-core`

-Rust引用使用下划线：`dtv_core::...`

-兼容性优先：

-`DanmakuFrontendPayload`在Phase1保持原名与字段不变（后续阶段再语义化重命名）

2.强约束（HardRules）

2.1dtv-core禁止依赖tauri

dtv-core内严格禁止出现：

-`tauri`crate依赖（Cargo.toml不得声明tauri）

-任意 `tauri::`路径引用

-任意Tauri宏：`#[tauri::command]`、`tauri::generate_handler!`等

-任意Tauri类型：`AppHandle`、`Window`、`State`、`Emitter`、`Manager`、`WebviewWindowBuilder`等

验收建议：

-`rg "tauri::|\\#\\[tauri::command\\]" dtv-core`必须无匹配

2.2Tauricommand必须在src-tauri（胶水层）

-所有 `#[tauri::command]`函数必须位于 `src-tauri/src/commands/**`（或同等明确的wrapper位置）

-dtv-core只提供普通Rust函数/async函数/structs/traits

2.3proxy.rs暂不迁移

-`src-tauri/src/proxy.rs`保持现状

-Phase1允许dtv-core返回upstreamurl；是否走代理与写入store等行为由src-tauri决定

3.分层边界（职责划分）

3.1dtv-core负责

-平台业务逻辑：

-API请求与解析（reqwest/serde_json）

-协议解包、弹幕解析、心跳、WS连接管理

-签名、哈希、加解密、编码转换

-统一的数据结构（如LiveStreamInfo、StreamVariant、DtvError等）

-运行时/工具：

-tokio、futures-util、tungstenite、prost、deno_core等（只要不依赖tauri）

3.2src-tauri负责

-窗口/插件/日志等应用装配（Tauribuilder）

-通过 `State<T>`管理生命周期对象（client、stophandle、store）

-把dtv-core的输出映射到前端协议：

-返回值结构保持一致

-弹幕事件 `emit("danmaku-message", payload)`

-B站cookie获取/静默窗口等Webview能力（属于壳层）

4.弹幕接口规范（Trait注入）

4.1Trait定义（dtv-core）

-在dtv-core定义：

-`trait DanmakuHandler: Send + Sync + 'static { fn on_danmaku(&self, msg: DanmakuFrontendPayload); }`

- dtv-core 内任何弹幕链路：
  - “解析完成 -> handler.on_danmaku(payload)”
  - 不允许直接 emit/调用 tauri

  4.2 Handler 实现（src-tauri）

- 在 src-tauri 实现一个 handler，把消息转发到前端：
  - `AppHandle.emit("danmaku-message", payload)`

- 事件名强约束：
  - Phase 1 固定为 `"danmaku-message"`（保持前端不变）

  4.3 停止/重启机制

- stop 控制必须不依赖 tauri：
  - dtv-core 可使用 `oneshot` / `mpsc` / 原子标志等

- stop handle 的存储位置：
  - Phase 1 推荐放在 `src-tauri` 的 managed state（例如 room_id -> stop_tx 的 HashMap）

5. API 函数签名迁移规范

   5.1 禁止将 `State<'_, T>`传入dtv-core

-迁移到dtv-core的函数参数必须是：

-`&T`/`Arc<T>`/普通值类型

-`State`的解包只允许发生在src-tauriwrapper内

5.2避免在dtv-core内创建散乱reqwestClient

-优先使用dtv-core内统一HttpClient（现仓库已有 `platforms/common/http_client.rs`逻辑可迁移并复用）

-如果必须 `reqwest::Client::builder()`：

- 给出明确理由（例如必须 no_proxy、必须独立 cookie jar、必须特殊 timeout）
- 不要在高频路径重复 build client（避免性能问题）

6. build.rs 与生成代码规范（prost）

6.1 build.rs 分离原则

- `dtv-core/build.rs`：只负责业务相关生成（prost）
- `src-tauri/build.rs`：只负责 `tauri_build::build()`

  6.2 生成代码位置

- Phase 1 建议继续将 proto 生成结果写入源码目录（例如 `dtv-core/src/platforms/douyin/danmu/gen`），并将生成文件纳入版本控制：
- 优点：CI/用户机器不需要 protoc 环境也能 build
- 保持现有 include 方式：`include!("douyin.rs")`

7. 错误处理与日志规范

- dtv-core：
- 对外返回 `Result<T, DtvError>` 或 `Result<T, String>`（推荐逐步统一到 `DtvError`，但 Phase 1 可保持现状以减少改动）
- 允许使用 `tracing` / `log`，但不依赖 tauri log plugin
- src-tauri：
- command 返回 `Result<..., String>`（与 tauri invoke 习惯一致）
- 负责把 dtv-core error 映射为用户可读信息（保持既有逻辑）

8. 迁移执行顺序建议（避免大爆炸）

推荐按“低耦合 -> 高耦合”顺序迁移：

1. 纯请求/解析模块（无 State/AppHandle/Window）
2. 把 `#[tauri::command]` 从平台模块剥离到 commands wrapper
3. 弹幕链路 trait 注入（优先改 douyin message_handler / douyu client / huya danmaku / bilibili danmaku）
4. 最后处理 `get_live_stream_v2`（proxy/store相关）

9.验收与守门（Gate）

每个PR/每个阶段必须满足：

-`dtv-core`下grep不到 `tauri`：

-`rg "tauri::|\\#\\[tauri::command\\]" dtv-core`无结果

-`cargo check -p dtv_core`通过

-`cargo check -p dtv`通过

-`pnpm tauri dev`启动后关键功能回归（至少：一个平台列表请求+一个弹幕平台推送）

10.后续扩展（不在Phase1）

-将 `DanmakuFrontendPayload`语义化为 `DanmakuMessage`，并用 `serde(rename)`/别名兼容前端字段

-将proxy抽到dtv-core的可选feature（例如 `features = ["proxy-actix"]`），并让src-tauri仅作为启停入口

-为dtv-core增加CLI或NAPI-RS绑定层（不影响核心库）
