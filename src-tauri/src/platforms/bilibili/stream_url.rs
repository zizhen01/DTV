use reqwest::header::{HeaderMap, HeaderValue, COOKIE, REFERER, USER_AGENT};
use tauri::{AppHandle, State};

use crate::proxy::{start_proxy, ProxyServerHandle, get_proxy_url};
use crate::StreamUrlStore;

use crate::platforms::common::errors::DtvError;

pub async fn get_bilibili_live_stream_url_with_quality(
    app_handle: AppHandle,
    stream_url_store: State<'_, StreamUrlStore>,
    proxy_server_handle: State<'_, ProxyServerHandle>,
    payload: crate::platforms::common::GetStreamUrlPayload,
    quality: String,
    cookie: Option<String>,
) -> Result<crate::platforms::common::LiveStreamInfo, DtvError> {
    let room_id = payload.args.room_id_str.clone();
    
    let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36";

    // Build headers
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str(ua).unwrap());
    headers.insert(
        REFERER,
        HeaderValue::from_static("https://live.bilibili.com/"),
    );
    if let Some(c) = cookie.as_ref() {
        let c_trimmed = c.trim();
        if !c_trimmed.is_empty() {
            match HeaderValue::from_str(c_trimmed) {
                Ok(val) => {
                    headers.insert(COOKIE, val);
                    tracing::debug!("[Bilibili] Cookie header set (content: {})", crate::platforms::common::logging::mask_sensitive(c_trimmed));
                }
                Err(err) => {
                    tracing::warn!("[Bilibili] Invalid cookie header, skipping. Error: {}", err);
                }
            }
        }
    }

    headers.insert(
        reqwest::header::ORIGIN,
        HeaderValue::from_static("https://live.bilibili.com"),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .no_proxy()
        .build()
        .map_err(|e| DtvError::internal(format!("Failed to build client: {}", e)))?;

    let mut info = dtv_core::platforms::bilibili::stream_url::get_bilibili_stream_url(
        &client,
        &room_id,
        &quality,
        cookie.as_deref()
    ).await?;

    if let Some(real_url) = info.stream_url.clone() {
        // FLV check: ends with .flv or contains .flv?
        // The core logic returned .flv or .m3u8 URLs.
        let is_flv = real_url.contains(".flv");

        if is_flv {
             // FLV: Start proxy
             {
                let mut urls = stream_url_store.urls.lock().unwrap();
                urls.insert(("bilibili".to_string(), room_id.clone()), real_url.clone());
            }
            match start_proxy(app_handle, proxy_server_handle, stream_url_store).await {
                Ok(_) => {
                    let proxy_url = get_proxy_url("bilibili", &room_id).await;
                    info.stream_url = Some(proxy_url);
                },
                Err(e) => {
                    tracing::error!("[Bilibili] Failed to start proxy: {}", e);
                    info.error_message = Some("代理启动失败".to_string());
                }
            }
        } else {
            // HLS: Remove from store (no proxy needed)
            {
                let mut urls = stream_url_store.urls.lock().unwrap();
                urls.remove(&("bilibili".to_string(), room_id.clone()));
            }
        }
    }

    Ok(info)
}