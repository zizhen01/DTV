use std::sync::Arc;

use tokio::sync::mpsc as tokio_mpsc;
use tokio::time::{sleep, Duration};

use crate::danmaku::DanmakuHandler;
use crate::platforms::douyin::danmu::web_fetcher::DouyinLiveWebFetcher;
use crate::platforms::douyin::danmu::{message_handler, websocket_connection};
use crate::platforms::douyin::web_api::normalize_douyin_live_id;

enum ConnectionOutcome {
    Stop,
    Disconnected,
}

pub async fn run_douyin_danmu_listener(
    room_id_or_url: String,
    mut rx_shutdown: tokio_mpsc::Receiver<()>,
    handler: Arc<dyn DanmakuHandler>,
) {
    if room_id_or_url == "stop_listening" {
        return;
    }

    let normalized_room_id = normalize_douyin_live_id(&room_id_or_url);
    let mut backoff_secs = 1u64;

    loop {
        let handler_clone = handler.clone();
        let room_id_clone = normalized_room_id.clone();

        let result: Result<ConnectionOutcome, String> = async {
            let mut fetcher = DouyinLiveWebFetcher::new(&room_id_clone)
                .map_err(|e| format!("Failed to init fetcher: {}", e))?;
            fetcher
                .fetch_room_details()
                .await
                .map_err(|e| format!("Failed to fetch room details: {}", e))?;

            let actual_room_id = fetcher.get_room_id().await.map_err(|e| e.to_string())?;
            let cookie_header = fetcher.get_dy_cookie().await.map_err(|e| e.to_string())?;
            let user_unique_id = fetcher.get_user_unique_id().await.map_err(|e| e.to_string())?;

            let (read_stream, ack_tx, shutdown_tx) = websocket_connection::connect_and_manage_websocket(
                &fetcher,
                &actual_room_id,
                &cookie_header,
                &user_unique_id,
            )
            .await
            .map_err(|e| e.to_string())?;

            let shutdown_tx_for_msg = shutdown_tx.clone();
            tokio::select! {
                res = message_handler::handle_received_messages(
                    read_stream,
                    ack_tx,
                    handler_clone,
                    actual_room_id.clone(),
                ) => {
                    let _ = shutdown_tx_for_msg.send(true);
                    res.map_err(|e| e.to_string())?;
                    Ok(ConnectionOutcome::Disconnected)
                }
                _ = rx_shutdown.recv() => {
                    let _ = shutdown_tx.send(true);
                    Ok(ConnectionOutcome::Stop)
                }
            }
        }
        .await;

        match result {
            Ok(ConnectionOutcome::Stop) => break,
            Ok(ConnectionOutcome::Disconnected) => {
                eprintln!(
                    "[Douyin Danmaku] Disconnected, retrying in {}s.",
                    backoff_secs
                );
            }
            Err(e) => {
                eprintln!(
                    "[Douyin Danmaku] Connection error: {}. Retrying in {}s.",
                    e, backoff_secs
                );
            }
        }

        let sleep_fut = sleep(Duration::from_secs(backoff_secs));
        tokio::select! {
            _ = sleep_fut => {}
            _ = rx_shutdown.recv() => break,
        }
        backoff_secs = (backoff_secs * 2).min(30);
    }
}
