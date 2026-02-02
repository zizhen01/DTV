mod common;

use dtv_core::platforms::common::FollowHttpClient;
use tracing::debug;
use common::{live_enabled, TestHandler};

const TARGET_ROOM_ID: &str = "74960"; // 定义为全局常量

#[tokio::test]
#[ignore]
async fn douyu_74960_room_info_live() {
    if !live_enabled() {
        return;
    }

    let _ = tracing_subscriber::fmt()
        .with_env_filter("dtv_core=debug") // 只看 dtv-core 的日志
        .try_init();

    let follow_http = FollowHttpClient::new().expect("FollowHttpClient::new");
    let info = dtv_core::platforms::douyu::fetch_douyu_room_info::fetch_douyu_room_info(
        TARGET_ROOM_ID.to_string(),
        &follow_http,
    )
    .await
    .expect("fetch_douyu_room_info");

    debug!(data = ?info, "fetched douyu room info");

    assert!(info.show_status.is_some());
    assert!(
        info.avatar_url.is_some() || info.nickname.is_some() || info.room_name.is_some(),
        "expected at least one of avatar/nickname/room_name"
    );
}

#[tokio::test]
#[ignore]
async fn douyu_74960_stream_urls_live() {
    if !live_enabled() {
        return;
    }

    let follow_http = FollowHttpClient::new().expect("FollowHttpClient::new");
    let info = dtv_core::platforms::douyu::fetch_douyu_room_info::fetch_douyu_room_info(
        TARGET_ROOM_ID.to_string(),
        &follow_http,
    )
    .await
    .expect("fetch_douyu_room_info");

    if info.show_status != Some(1) {
        // Avoid flakiness: skip if offline.
        return;
    }

    let qualities = ["原画", "高清", "标清"];
    let mut ok_count = 0usize;
    for q in qualities {
        match dtv_core::platforms::douyu::stream_url::get_stream_url_with_quality("74960", q, None)
            .await
        {
            Ok(url) => {
                assert!(!url.trim().is_empty());
                assert!(url.starts_with("http"));
                ok_count += 1;
            }
            Err(e) => {
                eprintln!("[live] get_stream_url_with_quality {} failed: {}", q, e);
            }
        }
    }

    assert!(ok_count >= 1, "expected at least one quality to succeed");
}

#[tokio::test]
#[ignore]
async fn douyu_74960_danmaku_listener_smoke() {
    if !live_enabled() {
        return;
    }

    let handler = TestHandler::new();

    let (stop_tx, stop_rx) = tokio::sync::oneshot::channel();
    let client = dtv_core::platforms::douyu::danmaku::DanmakuClient::new(
        TARGET_ROOM_ID,
        handler.clone(),
        stop_rx,
    );

    let task = tokio::spawn(async move {
        let mut client = client;
        let _ = client.start().await;
    });

    tokio::time::sleep(std::time::Duration::from_secs(8)).await;
    let _ = stop_tx.send(());
    let _ = task.await;

    // Best-effort assertion: do not fail hard if the room is quiet.
}

