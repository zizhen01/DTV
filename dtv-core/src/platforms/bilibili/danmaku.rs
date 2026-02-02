use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc as tokio_mpsc;

use crate::danmaku::DanmakuHandler;
use crate::platforms::bilibili::models::BiliMessage;
use crate::platforms::bilibili::websocket::BiliLiveClient;
use crate::platforms::common::DanmakuFrontendPayload;

pub async fn run_bilibili_danmaku_listener(
    room_id: String,
    cookie: Option<String>,
    mut rx_shutdown: tokio_mpsc::Receiver<()>,
    handler: Arc<dyn DanmakuHandler>,
) {
    // Use atomic flag to signal std::thread to stop
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_for_thread = stop_flag.clone();
    let handler_for_thread = handler.clone();
    let room_id_for_thread = room_id.clone();

    std::thread::spawn(move || {
        let mut client = match cookie.as_deref() {
            Some(c) => BiliLiveClient::new_with_cookie(c, room_id_for_thread.as_str()),
            None => BiliLiveClient::new_without_cookie(room_id_for_thread.as_str()),
        };
        client.send_auth();

        while !stop_flag_for_thread.load(Ordering::Relaxed) {
            if let Some(msg) = client.read_once() {
                match msg {
                    BiliMessage::Danmu { user, text } => {
                        handler_for_thread.on_danmaku(DanmakuFrontendPayload {
                            room_id: room_id_for_thread.clone(),
                            user,
                            content: text,
                            user_level: 0,
                            fans_club_level: 0,
                        });
                    }
                    BiliMessage::Gift { user, gift } => {
                        handler_for_thread.on_danmaku(DanmakuFrontendPayload {
                            room_id: room_id_for_thread.clone(),
                            user,
                            content: format!("[礼物] {}", gift),
                            user_level: 0,
                            fans_club_level: 0,
                        });
                    }
                    BiliMessage::Unsupported { .. } => {}
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    let _ = rx_shutdown.recv().await;
    stop_flag.store(true, Ordering::Relaxed);
}
