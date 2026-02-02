#[tauri::command]
pub fn generate_douyin_ms_token() -> String {
    dtv_core::platforms::douyin::danmu::signature::generate_douyin_ms_token()
}
