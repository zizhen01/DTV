use dtv_core::platforms::common::types::CommonPlatformCategory;

#[tauri::command]
pub async fn fetch_three_cate(tag_id: i32) -> Result<Vec<CommonPlatformCategory>, String> {
    dtv_core::platforms::douyu::three_cate::fetch_three_cate(tag_id).await
}
