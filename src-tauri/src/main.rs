// 在开发模式下允许控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use tauri::Manager;
mod commands;
mod platforms;
mod proxy;
use platforms::common::{DouyinDanmakuState, FollowHttpClient, HuyaDanmakuState};
// Douyin danmaku + msToken commands live in commands module
// use platforms::huya::get_huya_stream_url_with_quality; // removed in favor of unified cmd

#[derive(Default, Clone)]
pub struct StreamUrlStore {
    pub urls: Arc<Mutex<HashMap<(String, String), String>>>, // (platform, room_id) -> url
}

// search_anchor seems fine, assuming douyu::search_anchor is correct
#[tauri::command]
async fn search_anchor(keyword: String) -> Result<String, String> {
    platforms::douyu::perform_anchor_search(&keyword)
        .await
        .map_err(|e| e.to_string())
}

// Main function corrected
fn main() {
    // Create a new HTTP client instance to be managed by Tauri
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .no_proxy()
        .build()
        .expect("Failed to create reqwest client");
    let follow_http_client = FollowHttpClient::new().expect("Failed to create follow http client");

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .level(if std::env::var("DTV_DEBUG").map(|v| v == "1").unwrap_or(false) {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            })
            .targets([
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir { file_name: None }),
            ])
            .build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Apply macOS vibrancy to the main window when running on macOS
            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
                if let Some(window) = app.get_webview_window("main") {
                    match apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None) {
                        Ok(_) => tracing::info!("vibrancy applied successfully"),
                        Err(e) => tracing::error!("vibrancy error: {:?}", e),
                    }
                }
            }
            Ok(())
        })
        .manage(client) // Manage the reqwest client
        .manage(follow_http_client) // 专用关注刷新客户端，避免占用默认连接池
        .manage(commands::douyu_danmaku::DouyuDanmakuHandles::default()) // Manage DouyuDanmakuHandles
        .manage(DouyinDanmakuState::default()) // Manage DouyinDanmakuState
        .manage(HuyaDanmakuState::default()) // Manage HuyaDanmakuState
        .manage(platforms::common::BilibiliDanmakuState::default()) // Manage BilibiliDanmakuState
        .manage(StreamUrlStore::default())
        .manage(proxy::ProxyServerHandle::default())
        .manage(platforms::bilibili::state::BilibiliState::default())
        .invoke_handler(tauri::generate_handler![
            commands::live_stream_v2_cmd::get_live_stream_v2,
            search_anchor,
            commands::douyu_danmaku::start_danmaku_listener,
            commands::douyu_danmaku::stop_danmaku_listener,
            commands::douyin_danmaku::start_douyin_danmu_listener,
            commands::huya_danmaku::start_huya_danmaku_listener,
            commands::huya_danmaku::stop_huya_danmaku_listener,
            commands::bilibili_danmaku::start_bilibili_danmaku_listener,
            commands::bilibili_danmaku::stop_bilibili_danmaku_listener,
             proxy::stop_proxy,
             proxy::start_static_proxy_server,
             commands::douyu_categories::fetch_categories,
             commands::douyu_live_list::fetch_live_list,
             commands::douyu_live_list::fetch_live_list_for_cate3,
             commands::douyu_three_cate::fetch_three_cate,
               commands::douyin_ms_token::generate_douyin_ms_token,
                commands::douyin_streamer_list::fetch_douyin_partition_rooms,
                commands::huya_live_list::fetch_huya_live_list,
              platforms::bilibili::state::generate_bilibili_w_webid,
               commands::bilibili_live_list::fetch_bilibili_live_list,
              platforms::bilibili::cookie::get_bilibili_cookie,
              platforms::bilibili::cookie::bootstrap_bilibili_cookie,
               commands::bilibili_search::search_bilibili_rooms,
               commands::huya_search::search_huya_anchors,
         ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
