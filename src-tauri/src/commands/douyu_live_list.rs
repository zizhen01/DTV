use dtv_core::platforms::douyu::live_list::FrontendLiveListResponse;

#[tauri::command]
pub async fn fetch_live_list(offset: u32, cate2: String, limit: u32) -> FrontendLiveListResponse {
    dtv_core::platforms::douyu::live_list::fetch_live_list(offset, cate2, limit).await
}

#[tauri::command]
pub async fn fetch_live_list_for_cate3(
    cate3_id: String,
    page: u32,
    limit: u32,
) -> FrontendLiveListResponse {
    dtv_core::platforms::douyu::live_list::fetch_live_list_for_cate3(cate3_id, page, limit).await
}
