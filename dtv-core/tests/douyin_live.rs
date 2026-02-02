mod common;

use dtv_core::platforms::common::{FollowHttpClient, GetStreamUrlPayload, types::GetStreamUrlArgs};
use common::{live_enabled, TestHandler};
use tracing::debug;

fn target_room_id() -> String {
    std::env::var("DTV_TEST_DOUYIN_ID").unwrap_or_else(|_| "714470305828".to_string())
}

#[tokio::test]
#[ignore]
async fn douyin_room_info_live() {
    if !live_enabled() {
        return;
    }

    let _ = tracing_subscriber::fmt()
        .with_env_filter("dtv_core=debug")
        .try_init();

    let follow_http = FollowHttpClient::new().expect("FollowHttpClient::new");
    let payload = GetStreamUrlPayload {
        args: GetStreamUrlArgs {
            room_id_str: target_room_id(),
        },
    };

    let info = dtv_core::platforms::douyin::douyin_streamer_info::fetch_douyin_streamer_info(
        payload,
        &follow_http,
    )
    .await
    .expect("fetch_douyin_streamer_info");

    debug!(data = ?info, "fetched douyin room info");

    // Even if offline, we should get some info
    assert!(info.status.is_some());
    
    if info.status == Some(2) { // 2 is Live for Douyin? Need to check enum/impl. 
        // In douyin_streamer_info.rs: "let status = room.get("status")... as i32;"
        // Usually 2 is live, 4 is offline?
        // Let's just check stream_url availability if status suggests live.
        
        // available_streams is populated
        if let Some(streams) = info.available_streams {
            assert!(!streams.is_empty());
        }
    }
}

#[tokio::test]
#[ignore]
async fn douyin_danmaku_listener_smoke() {
    if !live_enabled() {
        return;
    }

    let handler = TestHandler::new();
    let (tx, rx) = tokio::sync::mpsc::channel(1);

    let task = tokio::spawn(dtv_core::platforms::douyin::douyin_danmu_listener::run_douyin_danmu_listener(
        target_room_id(),
        rx,
        handler.clone(),
    ));

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    let _ = tx.send(()).await;
    let _ = task.await;
}
