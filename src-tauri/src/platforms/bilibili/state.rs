pub use dtv_core::platforms::bilibili::state::BilibiliState;

#[tauri::command]
pub async fn generate_bilibili_w_webid(
    state: tauri::State<'_, BilibiliState>,
) -> Result<String, String> {
    dtv_core::platforms::bilibili::state::refresh_w_webid(state.inner()).await
}
