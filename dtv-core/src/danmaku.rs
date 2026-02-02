use crate::platforms::common::DanmakuFrontendPayload;

pub trait DanmakuHandler: Send + Sync + 'static {
    fn emit_json(&self, event: &str, payload: serde_json::Value);

    fn on_danmaku(&self, payload: DanmakuFrontendPayload) {
        match serde_json::to_value(payload) {
            Ok(v) => self.emit_json("danmaku-message", v),
            Err(_) => self.emit_json("danmaku-message", serde_json::Value::Null),
        }
    }
}
