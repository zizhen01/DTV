mod common;

use dtv_core::platforms::common::FollowHttpClient;
use common::{live_enabled, TestHandler};
use tracing::debug;

const TARGET_ROOM_ID: &str = "880201"; // LPL

#[tokio::test]
#[ignore]
async fn huya_880201_unified_live() {
    if !live_enabled() {
        return;
    }

    let _ = tracing_subscriber::fmt()
        .with_env_filter("dtv_core=debug")
        .try_init();

    let follow_http = FollowHttpClient::new().expect("FollowHttpClient::new");
    
    // Check info and stream in one go
    let resp = dtv_core::platforms::huya::stream_url::get_huya_unified_cmd(
        TARGET_ROOM_ID.to_string(),
        Some("原画".to_string()),
        None,
        &follow_http,
    )
    .await
    .expect("get_huya_unified_cmd");

    debug!(data = ?resp, "fetched huya unified info");

    assert!(resp.title.is_some() || resp.nick.is_some());
    
    if resp.is_live {
        assert!(!resp.flv_tx_urls.is_empty(), "Live but no stream urls");
        if let Some(url) = resp.selected_url {
            assert!(url.starts_with("http"));
        }
    }
}

#[tokio::test]
#[ignore]
async fn huya_880201_danmaku_listener_smoke() {
    if !live_enabled() {
        return;
    }

    let handler = TestHandler::new();
    let (tx, rx) = tokio::sync::mpsc::channel(1);

    let task = tokio::spawn(dtv_core::platforms::huya::danmaku::run_huya_danmaku_listener(
        TARGET_ROOM_ID.to_string(),
        rx,
        handler.clone(),
    ));

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    let _ = tx.send(()).await;
    let _ = task.await;
}
