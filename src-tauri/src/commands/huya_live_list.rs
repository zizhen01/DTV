use dtv_core::platforms::huya::live_list::HuyaLiveListFrontendResponse;

#[tauri::command]
pub async fn fetch_huya_live_list(
    i_gid: String,
    i_page_no: u32,
    i_page_size: u32,
) -> HuyaLiveListFrontendResponse {
    dtv_core::platforms::huya::live_list::fetch_huya_live_list(i_gid, i_page_no, i_page_size).await
}
