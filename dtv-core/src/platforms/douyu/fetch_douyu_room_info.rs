// Douyu specific API logic will go here
// NOTE: This module already uses reqwest and is consistent with the unified Douyu HTTP client approach.

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::platforms::common::errors::DtvError;
use crate::platforms::common::FollowHttpClient;
use tracing::{info, instrument};

// Define the structure to be returned to TypeScript
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DouyuFollowInfo {
    pub room_id: String,
    pub room_name: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub video_loop: Option<i64>,
    pub show_status: Option<i64>,
}

pub(crate) fn parse_douyu_room_info(
    input_room_id: &str,
    full_json_value: &Value,
) -> Result<DouyuFollowInfo, DtvError> {
    let room_data_ref = full_json_value
        .get("data")
        .and_then(|d| d.get("room"))
        .or_else(|| full_json_value.get("data"))
        .or_else(|| full_json_value.get("room"))
        .or_else(|| Some(full_json_value));

    let room_data = room_data_ref.ok_or_else(|| {
        DtvError::api(format!(
            "Could not locate room data block in JSON response for room {}",
            input_room_id
        ))
    })?;

    let get_str = |val: &Value, key: &str| val.get(key).and_then(|v| v.as_str()).map(String::from);
    let get_i64 = |val: &Value, key: &str| val.get(key).and_then(|v| v.as_i64());
    let get_nested_str = |val: &Value, path: &[&str]| {
        let mut current = val;
        for key_part in path.iter() {
            current = current.get(*key_part)?;
        }
        current.as_str().map(String::from)
    };

    let avatar_final_url = get_str(room_data, "avatar_mid")
        .or_else(|| get_nested_str(room_data, &["avatar", "middle"]));

    let final_room_id = get_str(room_data, "room_id").unwrap_or_else(|| input_room_id.to_string());

    Ok(DouyuFollowInfo {
        room_id: final_room_id,
        room_name: get_str(room_data, "room_name"),
        nickname: get_str(room_data, "nickname"),
        avatar_url: avatar_final_url,
        video_loop: get_i64(room_data, "videoLoop"),
        show_status: get_i64(room_data, "show_status"),
    })
}



#[instrument(
    skip(follow_http), 
    fields(room_id = %room_id), 
    err // 👈 关键：如果返回 Err，会自动打印一条 ERROR 日志，包含错误内容
)]
pub async fn fetch_douyu_room_info(
    room_id: String,
    follow_http: &FollowHttpClient,
) -> Result<DouyuFollowInfo, DtvError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Accept",
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("zh-CN,zh;q=0.9"),
    );
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    headers.insert("Pragma", HeaderValue::from_static("no-cache"));
    headers.insert(
        "Referer",
        HeaderValue::from_str(&format!("https://www.douyu.com/{}", room_id)).unwrap(),
    );
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36"));

    let response_result = follow_http
        .0
        .inner
        .get(format!("https://www.douyu.com/betard/{}", room_id))
        .headers(headers)
        .send()
        .await;

    let response = match response_result {
        Ok(res) => res,
        Err(e) => {
            return Err(DtvError::network(format!(
                "Network request failed for room {}: {}",
                room_id,
                e.to_string()
            )))
        }
    };

    if !response.status().is_success() {
        return Err(DtvError::api(format!(
            "API request for room {} failed with status: {}",
            room_id,
            response.status()
        )));
    }

    let full_json_value = match response.json::<Value>().await {
        Ok(val) => val,
        Err(e) => {
            return Err(DtvError::api(format!(
                "Failed to parse JSON for room {}: {}. Ensure API returns valid JSON.",
                room_id,
                e.to_string()
            )))
        }
    };
    let result = parse_douyu_room_info(&room_id, &full_json_value);

    if let Ok(ref info) = result {
    tracing::info!(
        // 使用 .as_deref() 可以把 Option<String> 变成 Option<&str>，打印更方便
        room_name = info.room_name.as_deref(),
        nickname = info.nickname.as_deref(),
        live = info.show_status == Some(1),
        "douyu_room_info_fetched"
    );
}

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_room_info_avatar_mid_path() {
        let input_room_id = "74960";
        let v = serde_json::json!({
            "data": {
                "room": {
                    "room_id": "74960",
                    "room_name": "test room",
                    "nickname": "tester",
                    "avatar_mid": "https://example.com/avatar.jpg",
                    "show_status": 1,
                    "videoLoop": 0
                }
            }
        });

        let info = parse_douyu_room_info(input_room_id, &v).unwrap();

        info!(info = ?info, "Parsed room info");
        assert_eq!(info.room_id, "74960");
        assert_eq!(info.room_name.as_deref(), Some("test room"));
        assert_eq!(info.nickname.as_deref(), Some("tester"));
        assert_eq!(
            info.avatar_url.as_deref(),
            Some("https://example.com/avatar.jpg")
        );
        assert_eq!(info.show_status, Some(1));
    }

    #[test]
    fn parse_room_info_avatar_nested_fallback() {
        let input_room_id = "74960";
        let v = serde_json::json!({
            "data": {
                "room_id": "74960",
                "room_name": "test room",
                "nickname": "tester",
                "avatar": { "middle": "https://example.com/avatar2.jpg" },
                "show_status": 0
            }
        });

        let info = parse_douyu_room_info(input_room_id, &v).unwrap();
        assert_eq!(
            info.avatar_url.as_deref(),
            Some("https://example.com/avatar2.jpg")
        );
        assert_eq!(info.show_status, Some(0));
    }

    #[test]
    fn parse_room_info_root_fallback_room_id_default() {
        let input_room_id = "74960";
        let v = serde_json::json!({
            "room_name": "test room",
            "nickname": "tester",
            "show_status": 1
        });

        let info = parse_douyu_room_info(input_room_id, &v).unwrap();
        assert_eq!(info.room_id, "74960");
        assert_eq!(info.show_status, Some(1));
    }
}
