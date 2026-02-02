use std::sync::{Arc, Mutex};
use dtv_core::danmaku::DanmakuHandler;

pub fn live_enabled() -> bool {
    std::env::var("DTV_LIVE_TEST")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub struct TestHandler {
    pub msgs: Mutex<Vec<serde_json::Value>>,
}

impl TestHandler {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            msgs: Mutex::new(Vec::new()),
        })
    }
}

impl DanmakuHandler for TestHandler {
    fn emit_json(&self, event: &str, payload: serde_json::Value) {
        if event != "danmaku-message" {
            return;
        }
        if let Ok(mut guard) = self.msgs.lock() {
            guard.push(payload);
        }
    }
}
