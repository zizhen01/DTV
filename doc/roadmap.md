roadmap.md

DTV Rust Core 解耦 Roadmap（Phase 1）
目标：把平台业务逻辑（爬虫、协议解析、加签、弹幕解包等）从 Tauri 壳层中解耦出来，形成不依赖 tauri 的纯 Rust 库 `dtv-core`（crate id: `dtv_core`），未来可复用于 NAPI-RS/Electron、独立 CLI 等场景；同时保持前端 invoke 协议不变，确保功能不回归。
背景现状（仓库扫描结果）

- 业务逻辑主要集中在 `src-tauri/src/platforms/**`，其中存在大量 `#[tauri::command]`、`tauri::State/AppHandle/Window/Emitter` 的耦合点。
- `src-tauri/src/main.rs` 仍承担部分业务编排（大量 platforms 直接引用 + 弹幕启动/停止 glue + invoke 列表非常长）。
- 抖音弹幕 proto 由 `src-tauri/build.rs` 使用 `prost_build` 生成到 `src-tauri/src/platforms/douyin/danmu/gen`，且同一个 build.rs 里还跑了 `tauri_build::build()`。
  总体策略
- `dtv-core`：承载所有可复用业务逻辑，不出现 `tauri` crate、`#[tauri::command]`、`tauri::State/AppHandle/Window/Emitter`。
- `src-tauri`：只保留 UI 绑定/窗口管理/插件初始化 + 命令包装（commands glue）+ 状态管理（stop handle 等）+ proxy（暂留）。
- 弹幕：在 `dtv-core` 定义 `DanmakuHandler` trait；Tauri 实现该 trait 并封装 `emit("danmaku-message", payload)`。
  里程碑与阶段拆分
  Phase 1.0：Workspace 引入（不改功能）
  交付物：
- 根目录新增 workspace `Cargo.toml`（虚拟清单）
- 新增 `dtv-core/` crate 基础骨架
  要点：
- workspace members = `["src-tauri", "dtv-core"]`
- `resolver = "2"`
- 将当前 `src-tauri/Cargo.toml` 里的 `[profile.release]` 移动到 workspace root（避免 profile 在 workspace 下失效或行为变化）
- `dtv-core` crate 名称为 `dtv-core`，crate id 为 `dtv_core`
  验收：
- `cargo check -p dtv` 通过（不要求功能已迁移）
- `cargo check -p dtv_core` 通过（空壳也可）

---

Phase 1.1：dtv-core 模块落地 + 目录迁移（compile 优先）
交付物：

- `dtv-core/src/platforms/**`：从 `src-tauri/src/platforms/**` 迁移而来（先迁移再逐步去 tauri 化）
- `dtv-core/src/lib.rs`：导出 `pub mod platforms;` 等
  策略：
- 优先“能编译、可渐进”：先把平台逻辑移到 dtv-core，再用 wrapper/适配器逐个拆掉 tauri 依赖点。
- 所有 `#[tauri::command]` 必须从 dtv-core 移除（改为普通函数），保留在 `src-tauri` wrapper 层。
  验收：
- `cargo check -p dtv_core` 通过
- 代码中 `dtv-core` 目录下不存在 `tauri::` 或 `#[tauri::command]`

---

Phase 1.2：弹幕通道解耦（Trait 注入）
交付物：

- `dtv_core::danmaku`（或 `dtv_core::platforms::common::danmaku`）新增：
  - `trait DanmakuHandler`
  - 如需停止控制：定义 `StopHandle`/`StopToken`（可以复用 oneshot/mpsc，但类型必须不依赖 tauri）
- 将以下弹幕逻辑改为“解析后调用 handler”，不直接 emit：
  - 斗鱼：原 `DanmakuClient` 持有 `tauri::Window`，改为持有 `Arc<dyn DanmakuHandler>`
  - 抖音：`message_handler` 当前 `AppHandle.emit`，改为 `handler.on_danmaku(...)`
  - 虎牙：`danmaku.rs` 当前 `AppHandle.emit`，改为 handler
  - B 站：当前 std thread 内 emit，改为 handler（若保持 std thread，也只调用 handler）
    Tauri glue（仍保留原 invoke 协议）：
- `src-tauri` 实现：
  - `struct TauriDanmakuHandler { app_handle: tauri::AppHandle, room_id: String }`
  - `impl DanmakuHandler for TauriDanmakuHandler { fn on_danmaku(&self, msg: DanmakuFrontendPayload) { let _ = self.app_handle.emit("danmaku-message", msg); } }`
- `src-tauri` 持有 stop sender/handle（例如 `DouyuDanmakuHandles` 这种 HashMap room_id -> stop_tx 仍可以留在壳层）
  验收：
- dtv-core 不再引用任何 tauri 类型
- 前端仍收到同名事件 `"danmaku-message"`，payload 结构不变（`DanmakuFrontendPayload`）

---

Phase 1.3：API/命令包装层重建（保持前端兼容）
交付物：

- `src-tauri/src/commands/**`：将所有 Tauri commands 集中在这里（薄 wrapper）
- `src-tauri/src/main.rs`：只做装配（manage state + register commands + plugins + window setup）
  原则：
- command 函数名保持不变（前端 invoke 不崩）
- wrapper 内部只做：
  - 参数校验/转换（最少）
  - 从 `State<T>` 取引用
  - 调 dtv-core 的纯函数
  - 管理 stop handle / emit handler
    优先迁移清单（建议顺序）：

1. 纯 HTTP/JSON 查询类：虎牙搜索、斗鱼分类、斗鱼直播列表等（改动小）
2. 直播流获取聚合：`get_live_stream_v2` 相关（依赖 proxy/store/state 较多，放后面）
3. 弹幕启动/停止：上一步已完成 handler 注入后接入 wrapper
   验收：

- `src-tauri/src/main.rs` 不再直接 `use platforms::...` 做大量业务编排（改为 use commands）
- `invoke_handler` 列表引用 `commands::*`，而不是直接引用 dtv_core 内部函数

---

Phase 1.4：build.rs 拆分（prost 归 core；tauri_build 归壳）
交付物：

- `dtv-core/build.rs`：只负责 `prost_build`（输出到 `dtv-core/src/platforms/douyin/danmu/gen`）
- `src-tauri/build.rs`：只保留 `tauri_build::build()`
  注意：
- 确保 proto 输出路径固定到源码目录（当前已有生成到 src 下的策略），以便 `include!("douyin.rs")` 继续工作
- 评估是否需要在 git 中提交生成文件（当前仓库内已存在 `gen/douyin.rs`，建议继续提交，确保无 protoc 环境也可 build）
  验收：
- `cargo check -p dtv_core` 可独立触发 proto 生成（或直接使用已生成代码）
- `cargo check -p dtv` 不再承担 prost 生成职责

---

Phase 1.5：稳定化与回归验证
回归 checklist：

- `pnpm tauri dev` 能启动
- 基础功能：
  - 获取直播列表/分类/搜索（斗鱼/虎牙/抖音/B站）
  - 获取直播流地址（含 debug variants / upstream_url 等字段保持一致）
  - 弹幕：斗鱼/抖音/虎牙/B站均能持续推送 `"danmaku-message"`
- dtv-core 约束检查：
  - 全仓 `dtv-core` 下无 `tauri::`、无 `#[tauri::command]`

---

风险与应对

- 风险：现有平台模块里混用了 `tauri::State`（例如 `fetch_douyu_room_info`, `huya/stream_url`），迁移时容易“参数风暴”
  - 应对：dtv-core API 统一改成 `fn(..., follow_http: &FollowHttpClient)` / `fn(..., client: &reqwest::Client)` 等引用形式；Tauri wrapper 再从 State 取引用传入
- 风险：B 站 cookie/bootstrap 逻辑深度依赖 Webview（这本质上就是壳层能力）
  - 应对：Phase 1 不迁移 B 站 cookie 获取窗口逻辑到 dtv-core；只把“需要 cookie 的网络请求/解析逻辑”放入 dtv-core
- 风险：`get_live_stream_v2` 强依赖 proxy/store
  - 应对：Phase 1 将其拆成两段：dtv-core 负责“拿到 upstream url + meta/variants”，src-tauri 负责“写入 store + 启动 proxy + 返回代理地址”

---

Phase 1 完成定义（Definition of Done）

- `dtv-core`（`dtv_core`）存在且可被其他 Rust 二进制复用
- dtv-core 不依赖 tauri，不包含任何 tauri 宏/类型
- Tauri 侧前端协议稳定：invoke 名称不变，弹幕事件名/结构不变
- 主要业务逻辑（平台解析、加签、协议解包）位于 `dtv-core`
- `src-tauri/src/main.rs` 仅装配，不承载平台业务实现

---
