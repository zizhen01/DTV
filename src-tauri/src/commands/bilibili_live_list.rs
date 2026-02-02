#[tauri::command]
pub async fn fetch_bilibili_live_list(
    area_id: String,
    parent_area_id: String,
    page: u32,
    state: tauri::State<'_, dtv_core::platforms::bilibili::state::BilibiliState>,
) -> Result<String, String> {
    dtv_core::platforms::bilibili::live_list::fetch_bilibili_live_list(
        area_id,
        parent_area_id,
        page,
        state.inner(),
    )
    .await
}
