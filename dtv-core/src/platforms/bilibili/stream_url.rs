use serde_json::Value;

use crate::platforms::common::types::StreamVariant;
use crate::platforms::common::errors::DtvError;
use crate::platforms::common::LiveStreamInfo;

pub async fn get_bilibili_stream_url(
    client: &reqwest::Client,
    room_id: &str,
    quality: &str,
    _cookie: Option<&str>,
) -> Result<LiveStreamInfo, DtvError> {
    if room_id.trim().is_empty() {
        return Ok(LiveStreamInfo {
            title: None,
            anchor_name: None,
            avatar: None,
            stream_url: None,
            status: None,
            error_message: Some("房间ID未提供".to_string()),
            upstream_url: None,
            available_streams: None,
            normalized_room_id: None,
            web_rid: None,
        });
    }

    // Helper: request playinfo with optional qn
    async fn request_playinfo(
        client: &reqwest::Client,
        room_id: &str,
        qn: Option<i32>,
    ) -> Result<Value, DtvError> {
        let url = "https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo";
        let mut params = vec![
            ("room_id", room_id.to_string()),
            ("protocol", "0,1".to_string()),
            ("format", "0,1,2".to_string()),
            ("codec", "0".to_string()),
            ("platform", "html5".to_string()),
            ("dolby", "5".to_string()),
        ];
        if let Some(q) = qn {
            params.push(("qn", q.to_string()));
        }
        let resp = client
            .get(url)
            .query(&params)
            .send()
            .await
            .map_err(|e| DtvError::network(format!("PlayInfo request failed: {}", e)))?;
        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|e| DtvError::network(format!("Read text failed: {}", e)))?;
        if !status.is_success() {
            return Err(DtvError::api(format!("PlayInfo status: {} body: {}", status, text)));
        }
        serde_json::from_str::<Value>(&text)
            .map_err(|e| DtvError::api(format!("JSON parse failed: {} | body: {}", e, text)))
    }

    // 1) First request to get qn mapping
    let playinfo = request_playinfo(client, room_id, None).await?;
    let playurl = playinfo["data"]["playurl_info"]["playurl"].clone();

    // Build qn->desc map
    let mut qn_map: Vec<(i32, String)> = vec![];
    if let Some(arr) = playurl.get("g_qn_desc").and_then(|v| v.as_array()) {
        for item in arr {
            let qn = item.get("qn").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let desc = item
                .get("desc")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            qn_map.push((qn, desc));
        }
    }

    // Choose qn by desired quality text
    fn match_qn(qn_map: &[(i32, String)], quality: &str) -> Option<i32> {
        let q = quality.trim();
        let mut qns: Vec<i32> = qn_map.iter().map(|(qn, _)| *qn).collect();
        qns.sort();
        let has = |v: i32| qns.binary_search(&v).is_ok();

        match q {
            "原画" => {
                if has(10000) {
                    Some(10000)
                } else {
                    qns.last().copied()
                }
            }
            "高清" => {
                if has(400) {
                    return Some(400);
                }
                for (qn, desc) in qn_map.iter() {
                    if desc.contains("高清") || desc.contains("超清") || desc.contains("HD") {
                        return Some(*qn);
                    }
                }
                let max = qns.last().copied();
                if let Some(m) = max {
                    qns.into_iter().rev().find(|&x| x < m)
                } else {
                    None
                }
            }
            "标清" => {
                if has(250) {
                    return Some(250);
                }
                for (qn, desc) in qn_map.iter() {
                    if desc.contains("标清") || desc.contains("流畅") || desc.contains("SD") {
                        return Some(*qn);
                    }
                }
                qns.first().copied()
            }
            _ => qns.last().copied(),
        }
    }

    let selected_qn = match_qn(&qn_map, quality);
    let selected_desc = selected_qn.and_then(|qn| {
        qn_map
            .iter()
            .find(|(q, _)| *q == qn)
            .map(|(_, d)| d.clone())
    });

    // Determine live status from room_init
    let room_init_url = format!(
        "https://api.live.bilibili.com/room/v1/Room/room_init?id={}",
        room_id
    );
    let init_resp = client
        .get(&room_init_url)
        .send()
        .await
        .map_err(|e| DtvError::network(format!("room_init failed: {}", e)))?;
    let init_text = init_resp
        .text()
        .await
        .map_err(|e| DtvError::network(format!("room_init read text failed: {}", e)))?;
    let init_json: Value = serde_json::from_str(&init_text)
        .map_err(|e| DtvError::api(format!("room_init json failed: {} | {}", e, init_text)))?;
    let live_status = init_json["data"]["live_status"].as_i64().unwrap_or(0);
    
    if live_status != 1 {
        return Ok(LiveStreamInfo {
            title: init_json["data"]["title"].as_str().map(|s| s.to_string()),
            anchor_name: init_json["data"]["uname"].as_str().map(|s| s.to_string()),
            avatar: None,
            stream_url: None,
            status: Some(0),
            error_message: None,
            upstream_url: None,
            available_streams: None,
            normalized_room_id: None,
            web_rid: None,
        });
    }

    enum SelectedStream {
        Flv(String),
        Hls(String),
    }

    fn parse_stream_variants(
        playurl: &Value,
        selected_desc: &Option<String>,
        selected_qn: Option<i32>,
    ) -> (Vec<StreamVariant>, Option<String>, Vec<String>) {
        let mut variants: Vec<StreamVariant> = Vec::new();
        let mut hls_candidates: Vec<String> = Vec::new();
        let mut flv_candidate: Option<String> = None;

        if let Some(streams) = playurl.get("stream").and_then(|v| v.as_array()) {
            for stream_item in streams {
                let protocol_name = stream_item
                    .get("protocol_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                if let Some(formats) = stream_item.get("format").and_then(|v| v.as_array()) {
                    for format_item in formats {
                        let format_name = format_item
                            .get("format_name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        if let Some(codecs) = format_item.get("codec").and_then(|v| v.as_array()) {
                            for codec_item in codecs {
                                let base_url = codec_item
                                    .get("base_url")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("");
                                if let Some(url_infos) =
                                    codec_item.get("url_info").and_then(|v| v.as_array())
                                {
                                    for ui in url_infos {
                                        let host =
                                            ui.get("host").and_then(|v| v.as_str()).unwrap_or("");
                                        let extra =
                                            ui.get("extra").and_then(|v| v.as_str()).unwrap_or("");
                                        let composed = format!("{}{}{}", host, base_url, extra);
                                        if composed.is_empty() {
                                            continue;
                                        }

                                        variants.push(StreamVariant {
                                            url: composed.clone(),
                                            format: Some(format_name.to_string()),
                                            desc: selected_desc.clone(),
                                            qn: selected_qn,
                                            protocol: if protocol_name.is_empty() {
                                                None
                                            } else {
                                                Some(protocol_name.clone())
                                            },
                                        });

                                        let is_hls_format = matches!(
                                            format_name,
                                            "ts" | "fmp4" | "mp4" | "m4s" | "m3u8"
                                        );
                                        let is_hls_protocol = protocol_name.contains("hls");
                                        if is_hls_format || is_hls_protocol {
                                            hls_candidates.push(composed.clone());
                                        }
                                        if format_name == "flv" && flv_candidate.is_none() {
                                            flv_candidate = Some(composed.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        (variants, flv_candidate, hls_candidates)
    }

    async fn verify_hls_candidates(
        client: &reqwest::Client,
        candidates: &[String],
    ) -> Option<String> {
        for candidate in candidates.iter().take(4) {
            match client.get(candidate).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return Some(candidate.clone());
                    }
                }
                Err(_) => {}
            }
        }
        None
    }

    const MAX_HLS_RETRY: usize = 3;
    let mut selected_stream: Option<SelectedStream> = None;
    let mut variants_for_response: Vec<StreamVariant> = Vec::new();
    let mut fallback_hls_url: Option<String> = None;
    let mut fallback_variants: Option<Vec<StreamVariant>> = None;

    for attempt in 0..=MAX_HLS_RETRY {
        let playinfo_attempt = request_playinfo(client, room_id, selected_qn).await?;
        let playurl_attempt = playinfo_attempt["data"]["playurl_info"]["playurl"].clone();
        let (variants, flv_candidate, hls_candidates) =
            parse_stream_variants(&playurl_attempt, &selected_desc, selected_qn);

        variants_for_response = variants.clone();

        if let Some(flv_url) = flv_candidate {
            selected_stream = Some(SelectedStream::Flv(flv_url));
            break;
        }

        if hls_candidates.is_empty() {
            if attempt == MAX_HLS_RETRY {
                break;
            }
            continue;
        }

        let (preferred_candidates, other_candidates): (Vec<String>, Vec<String>) = hls_candidates
            .into_iter()
            .partition(|url| url.contains("d1--cn"));

        if let Some(url) = verify_hls_candidates(client, &preferred_candidates).await {
            selected_stream = Some(SelectedStream::Hls(url));
            break;
        }

        if fallback_hls_url.is_none() {
            if let Some(url) = verify_hls_candidates(client, &other_candidates).await {
                fallback_hls_url = Some(url.clone());
                fallback_variants = Some(variants.clone());
            }
        }

        if attempt == MAX_HLS_RETRY {
            if let Some(url) = fallback_hls_url.clone() {
                selected_stream = Some(SelectedStream::Hls(url));
                if let Some(fallback) = fallback_variants.clone() {
                    variants_for_response = fallback;
                }
            } else if let Some(url) = verify_hls_candidates(client, &other_candidates).await {
                selected_stream = Some(SelectedStream::Hls(url));
                variants_for_response = variants.clone();
            }
        }
    }

    if selected_stream.is_none() {
        if let Some(url) = fallback_hls_url.clone() {
            selected_stream = Some(SelectedStream::Hls(url));
            if let Some(fallback) = fallback_variants.clone() {
                variants_for_response = fallback;
            }
        }
    }

    let selected_stream = match selected_stream {
        Some(stream) => stream,
        None => {
            return Ok(LiveStreamInfo {
                title: init_json["data"]["title"].as_str().map(|s| s.to_string()),
                anchor_name: init_json["data"]["uname"].as_str().map(|s| s.to_string()),
                avatar: None,
                stream_url: None,
                status: Some(2),
                error_message: Some("未找到可用的直播流地址".to_string()),
                upstream_url: None,
                available_streams: Some(variants_for_response),
                normalized_room_id: None,
                web_rid: None,
            });
        }
    };

    match selected_stream {
        SelectedStream::Flv(real_url) => Ok(LiveStreamInfo {
            title: init_json["data"]["title"].as_str().map(|s| s.to_string()),
            anchor_name: init_json["data"]["uname"].as_str().map(|s| s.to_string()),
            avatar: None,
            stream_url: Some(real_url.clone()),
            status: Some(1),
            error_message: None,
            upstream_url: Some(real_url),
            available_streams: Some(variants_for_response),
            normalized_room_id: None,
            web_rid: None,
        }),
        SelectedStream::Hls(real_url) => Ok(LiveStreamInfo {
            title: init_json["data"]["title"].as_str().map(|s| s.to_string()),
            anchor_name: init_json["data"]["uname"].as_str().map(|s| s.to_string()),
            avatar: None,
            stream_url: Some(real_url.clone()),
            status: Some(1),
            error_message: None,
            upstream_url: Some(real_url),
            available_streams: Some(variants_for_response),
            normalized_room_id: None,
            web_rid: None,
        }),
    }
}