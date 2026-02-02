use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::oneshot;
use tauri::Emitter;

// State for managing Douyu danmaku listener handles (stop signals)
#[derive(Default, Clone)]
pub struct DouyuDanmakuHandles(pub Arc<Mutex<HashMap<String, oneshot::Sender<()>>>>);

#[tauri::command]
pub async fn start_danmaku_listener(
    room_id: String,
    window: tauri::Window,
    danmaku_handles: tauri::State<'_, DouyuDanmakuHandles>,
) -> Result<(), String> {
    if let Some(existing_sender) = danmaku_handles.0.lock().unwrap().remove(&room_id) {
        let _ = existing_sender.send(());
    }

    let (stop_tx, stop_rx) = oneshot::channel();
    danmaku_handles
        .0
        .lock()
        .unwrap()
        .insert(room_id.clone(), stop_tx);

    struct TauriWindowDanmakuHandler {
        window: tauri::Window,
    }

    impl dtv_core::danmaku::DanmakuHandler for TauriWindowDanmakuHandler {
        fn emit_json(&self, event: &str, payload: serde_json::Value) {
            let _ = self.window.emit(event, payload);
        }
    }

    let handler: Arc<dyn dtv_core::danmaku::DanmakuHandler> = Arc::new(TauriWindowDanmakuHandler {
        window: window.clone(),
    });

    let room_id_clone = room_id.clone();
    tokio::spawn(async move {
        let mut client = dtv_core::platforms::douyu::danmaku::DanmakuClient::new(
            &room_id_clone,
            handler,
            stop_rx,
        );
        if let Err(e) = client.start().await {
            eprintln!(
                "[Douyu Danmaku] client for room {} failed: {}",
                room_id_clone, e
            );
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_danmaku_listener(
    room_id: String,
    danmaku_handles: tauri::State<'_, DouyuDanmakuHandles>,
) -> Result<(), String> {
    if let Some(sender) = danmaku_handles.0.lock().unwrap().remove(&room_id) {
        match sender.send(()) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!(
                "Failed to stop Douyu danmaku listener for room {}: receiver dropped.",
                room_id
            )),
        }
    } else {
        Ok(())
    }
}
