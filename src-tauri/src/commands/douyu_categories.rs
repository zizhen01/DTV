use dtv_core::platforms::douyu::fetch_douyu_main_categories::CategoriesApiResponse;

#[tauri::command]
pub async fn fetch_categories() -> Result<CategoriesApiResponse, String> {
    dtv_core::platforms::douyu::fetch_douyu_main_categories::fetch_categories().await
}
