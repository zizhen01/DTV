# dtv-core Testing Plan

This document describes the test plan for `dtv-core`, with an initial focus on Douyu (`room_id=74960`).

Guiding principles:

- Prefer deterministic unit tests (fixtures, pure functions) for CI stability.
- Treat real-network tests as optional “live integration tests” gated by `#[ignore]` and an env var.
- Avoid asserting brittle values that can change (viewer count, title, whether the room is live, number of danmaku messages).

## Test tiers

### 1) Unit tests (deterministic)

Characteristics:

- No real network access.
- Use local fixtures (e.g., JSON snapshots) and pure parsing/mapping functions.
- Always run in CI.

Location:

- Prefer `#[cfg(test)] mod tests` inside the module being tested.
- For shared fixtures, place them under `dtv-core/tests/fixtures/**`.

### 2) Live integration tests (network)

Characteristics:

- Talks to real endpoints and WebSocket services.
- Potentially flaky due to rate limits, geo restrictions, room offline state, and network variance.
- Marked `#[ignore]` and optionally also gated by `DTV_LIVE_TEST=1`.

Location:

- `dtv-core/tests/**` integration test files.

Recommended invocation:

```bash
cargo test -p dtv-core
DTV_LIVE_TEST=1 cargo test -p dtv-core -- --ignored --nocapture
```

## Douyu: Room info (avatar, live status, basic fields)

Target module:

- `dtv-core/src/platforms/douyu/fetch_douyu_room_info.rs`

### A1. Parse betard JSON (unit, fixture)

Goal:

- Verify that `avatar_url`, `show_status`, `room_name`, `nickname` extraction is robust to response shape changes.

Plan:

- Introduce a pure function (recommended):
  - `parse_douyu_room_info(input_room_id: &str, json: &serde_json::Value) -> Result<DouyuFollowInfo, DtvError>`
- Keep `fetch_douyu_room_info()` as thin I/O wrapper: fetch -> json -> parse.

Fixtures:

- `dtv-core/tests/fixtures/douyu/betard_74960.json` (captured from `https://www.douyu.com/betard/74960`).

Assertions (stable):

- `room_id` is present and equals input or server-provided `room_id`.
- `avatar_url` is extracted from (in priority order):
  - `avatar_mid`
  - `avatar.middle`
- `show_status` parses as `Some(i64)`.
- `room_name`/`nickname` parsing does not panic and returns `Option<String>`.

### A2. Live: fetch_douyu_room_info(74960) (integration)

Goal:

- Verify that the request works in a real environment and returns at least some data.

Constraints:

- Room can be offline.
- Fields can be missing.

Assertions (loose):

- Call succeeds OR fails with a recognized network/API error (no panic).
- If it succeeds: `show_status.is_some()`.
- If it succeeds: at least one of `avatar_url`, `nickname`, `room_name` is `Some`.

## Douyu: Stream URL (qualities)

Target module:

- `dtv-core/src/platforms/douyu/stream_url.rs`

Public API today:

- `get_stream_url_with_quality(room_id: &str, quality: &str, cdn: Option<&str>) -> Result<String, DtvError>`

### B1. Quality mapping resolver (unit)

Goal:

- Ensure `resolve_rate_for_quality()` continues to map user-facing quality labels to correct rates.

Plan:

- Test the mapping using a constructed `variants` list (no network):
  - includes common rates (e.g., 0/3/4) and names containing `原画/高清/标清/蓝光/超清/流畅`.

Assertions:

- `"原画"` returns rate 0 if present.
- `"高清"` returns rate 4 if present (or reasonable fallback among non-0 variants).
- `"标清"` returns rate 3 if present (or reasonable fallback among non-0 variants).
- English aliases: `origin/high/standard` map to the same semantics.

### B2. Live: stream URL for multiple qualities (integration)

Goal:

- Verify that when the room is live, we can obtain stream URLs.

Plan:

- First check room live status via betard (`fetch_douyu_room_info` or internal betard logic).
- If offline: skip (do not fail) to avoid flakiness.
- If live: call for a small set of qualities:
  - `原画`, `高清`, `标清`.

Assertions (loose):

- Returned URLs are non-empty.
- Returned URLs look like URLs (`starts_with("http")`).

### B3. List all available quality variants (future improvement)

Current status:

- `stream_url.rs` has internal `get_play_qualities()` which yields `multirates`, but there is no public “list qualities” API.

Recommendation:

- Add a new pure API:
  - `get_available_stream_variants(room_id: &str) -> Result<Vec<StreamVariant>, DtvError>`
- Then add a live integration test:
  - variants list is non-empty when live.

## Douyu: Danmaku (chat messages)

Target module:

- `dtv-core/src/platforms/douyu/danmaku.rs`

Note:

- Douyu danmaku is a continuous high-frequency stream. The core library provides a listener (`DanmakuClient`) which pushes events via `DanmakuHandler`.
- There is no “danmaku list” REST endpoint in the current design.

### C1. Frame parsing unit tests (recommended refactor)

Goal:

- Make danmaku parsing testable without network.

Plan:

- Extract the parsing logic into a pure function:
  - `parse_douyu_frame(data: &[u8], room_id: &str) -> Option<DanmakuFrontendPayload>`
- Add binary fixtures (captured frames) under:
  - `dtv-core/tests/fixtures/douyu/danmaku_chatmsg.bin` (and a few variants)

Assertions:

- For a valid `chatmsg`, payload has non-empty `user` and `content`.
- Non-chat messages return `None`.
- Invalid frames do not panic.

### C2. Live: start listener and collect messages (integration)

Goal:

- Verify the listener can connect, run briefly, and stop cleanly.

Plan:

- If room is offline: skip.
- Start `DanmakuClient` with a test handler that collects messages.
- Run for 5-10 seconds.
- Send stop signal.

Assertions (loose):

- No panic.
- Task stops cleanly.
- Optionally (best-effort): collected message count > 0.

## Environment and reliability notes

- Live tests may fail due to:
  - room offline
  - rate limiting / CAPTCHA
  - regional restrictions
  - network variability
- Therefore, live tests should be opt-in.
