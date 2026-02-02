use dtv_core::platforms::bilibili::search::BilibiliSearchItem;

#[tauri::command]
pub async fn search_bilibili_rooms(
    keyword: String,
    page: Option<u32>,
    cookie: Option<String>,
) -> Result<Vec<BilibiliSearchItem>, String> {
    dtv_core::platforms::bilibili::search::search_bilibili_rooms(keyword, page, cookie).await
}
