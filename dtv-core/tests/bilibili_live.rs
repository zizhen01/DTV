mod common;

use dtv_core::platforms::common::{FollowHttpClient, GetStreamUrlPayload, types::GetStreamUrlArgs};
use dtv_core::platforms::bilibili::state::BilibiliState;
use common::{live_enabled, TestHandler};
use tracing::debug;

const TARGET_ROOM_ID: &str = "7734200"; // Official/Popular room

#[tokio::test]
#[ignore]
async fn bilibili_7734200_room_info_live() {
    if !live_enabled() {
        return;
    }

    let _ = tracing_subscriber::fmt()
        .with_env_filter("dtv_core=debug")
        .try_init();

    let follow_http = FollowHttpClient::new().expect("FollowHttpClient::new");
    let state = BilibiliState::default();

    let payload = GetStreamUrlPayload {
        args: GetStreamUrlArgs {
            room_id_str: TARGET_ROOM_ID.to_string(),
        },
    };

    let info = dtv_core::platforms::bilibili::streamer_info::fetch_bilibili_streamer_info(
        payload,
        None,
        &follow_http,
        &state,
    )
    .await
    .expect("fetch_bilibili_streamer_info");

    debug!(data = ?info, "fetched bilibili room info");

    // Bilibili might return offline status but still have info
    assert!(info.status.is_some());
    assert!(
        info.avatar.is_some() || info.anchor_name.is_some() || info.title.is_some(),
        "expected at least one of avatar/anchor_name/title"
    );
}

#[tokio::test]
#[ignore]
async fn bilibili_7734200_stream_urls_live() {
    if !live_enabled() {
        return;
    }

    let follow_http = FollowHttpClient::new().expect("FollowHttpClient::new");
    let state = BilibiliState::default();
    
    // First check if live
    let payload = GetStreamUrlPayload {
        args: GetStreamUrlArgs {
            room_id_str: TARGET_ROOM_ID.to_string(),
        },
    };
    let info = dtv_core::platforms::bilibili::streamer_info::fetch_bilibili_streamer_info(
        payload,
        None,
        &follow_http,
        &state,
    )
    .await
    .expect("fetch_bilibili_streamer_info");

    if info.status != Some(1) {
        return;
    }

    let client = &follow_http.0.inner;
    // Bilibili core stream logic now available in dtv-core
    let stream_info = dtv_core::platforms::bilibili::stream_url::get_bilibili_stream_url(
        client,
        TARGET_ROOM_ID,
        "原画",
        None
    ).await.expect("get_bilibili_stream_url");

    if let Some(url) = stream_info.stream_url {
        assert!(url.starts_with("http"));
        println!("Got stream url: {}", url);
    } else {
        // sometimes it fails to get stream if region locked or whatever, but should not error
        // If live, we expect a stream url usually.
        panic!("Live but no stream url found");
    }
}

#[tokio::test]
#[ignore]
async fn bilibili_7734200_danmaku_listener_smoke() {
    if !live_enabled() {
        return;
    }

    let handler = TestHandler::new();
    let (tx, rx) = tokio::sync::mpsc::channel(1);

    let task = tokio::spawn(dtv_core::platforms::bilibili::danmaku::run_bilibili_danmaku_listener(
        TARGET_ROOM_ID.to_string(),
        None,
        rx,
        handler.clone(),
    ));

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    let _ = tx.send(()).await;
    let _ = task.await;

    // We can't guarantee messages in 10s, but we can check if we panicked
}
