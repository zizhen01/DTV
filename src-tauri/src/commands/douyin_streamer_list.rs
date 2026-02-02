use dtv_core::platforms::douyin::douyin_streamer_list::DouyinLiveListResponse;

#[tauri::command]
pub async fn fetch_douyin_partition_rooms(
    partition: String,
    partition_type: String,
    offset: i32,
    ms_token: String,
) -> Result<DouyinLiveListResponse, String> {
    dtv_core::platforms::douyin::douyin_streamer_list::fetch_douyin_partition_rooms(
        partition,
        partition_type,
        offset,
        ms_token,
    )
    .await
}
