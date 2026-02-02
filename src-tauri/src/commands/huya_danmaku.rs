use std::sync::Arc;

use tauri::Emitter;
use tokio::sync::mpsc as tokio_mpsc;

use dtv_core::danmaku::DanmakuHandler;

#[tauri::command]
pub async fn start_huya_danmaku_listener(
    payload: dtv_core::platforms::common::GetStreamUrlPayload,
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, dtv_core::platforms::common::HuyaDanmakuState>,
) -> Result<(), String> {
    let room_id_or_url = payload.args.room_id_str;

    let previous_tx = {
        let mut lock = state.inner().0.lock().unwrap();
        lock.take()
    };
    if let Some(tx) = previous_tx {
        let _ = tx.send(()).await;
    }

    let (tx_shutdown, rx_shutdown) = tokio_mpsc::channel::<()>(1);
    {
        let mut lock = state.inner().0.lock().unwrap();
        *lock = Some(tx_shutdown);
    }

    struct TauriAppDanmakuHandler {
        app_handle: tauri::AppHandle,
    }

    impl DanmakuHandler for TauriAppDanmakuHandler {
        fn emit_json(&self, event: &str, payload: serde_json::Value) {
            let _ = self.app_handle.emit(event, payload);
        }
    }

    let handler: Arc<dyn DanmakuHandler> = Arc::new(TauriAppDanmakuHandler {
        app_handle: app_handle.clone(),
    });

    tokio::spawn(async move {
        dtv_core::platforms::huya::danmaku::run_huya_danmaku_listener(
            room_id_or_url,
            rx_shutdown,
            handler,
        )
        .await;
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_huya_danmaku_listener(
    room_id: String,
    state: tauri::State<'_, dtv_core::platforms::common::HuyaDanmakuState>,
) -> Result<(), String> {
    let _ = room_id;
    let tx = {
        let mut lock = state.inner().0.lock().unwrap();
        lock.take()
    };
    if let Some(tx) = tx {
        let _ = tx.send(()).await;
    }
    Ok(())
}
