use std::{
    sync::{mpsc, OnceLock},
    thread,
};

use deno_core::{FastString, JsRuntime, RuntimeOptions};
use tokio::sync::oneshot;

use crate::platforms::common::js_function_worker::JsFunctionWorker;
use crate::platforms::common::js_runtime::ensure_js_runtime_platform_initialized;

const CRYPTO_JS: &str = include_str!("cryptojs.min.js");

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SignerMode {
    Unified,
    Legacy,
}

// One-line switch:
const SIGNER_MODE: SignerMode = SignerMode::Unified;

const DOUYU_SIGN_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";

mod legacy {
    use super::*;

    struct SignJob {
        script: String,
        rid: String,
        did: String,
        ts: i64,
        respond_to: oneshot::Sender<Result<String, String>>,
    }

    pub(super) struct LegacyDouyuSignerWorker {
        tx: mpsc::Sender<SignJob>,
    }

    impl LegacyDouyuSignerWorker {
        fn new() -> Result<Self, String> {
            let (tx, rx) = mpsc::channel::<SignJob>();

            thread::Builder::new()
                .name("douyu-signer-worker".to_string())
                .spawn(move || {
                    ensure_js_runtime_platform_initialized();

                    while let Ok(job) = rx.recv() {
                        let response = (|| {
                            // Match douyu_sample semantics strictly:
                            // - new JsRuntime per call
                            // - execute cryptojs, then the per-room script, then call ub98484234
                            // - no window/navigator/document bootstrap
                            let mut runtime = JsRuntime::new(RuntimeOptions::default());

                            runtime
                                .execute_script(
                                    "[douyu-cryptojs]",
                                    FastString::from_static(CRYPTO_JS),
                                )
                                .map_err(|e| format!("js cryptojs execute failed: {e}"))?;

                            runtime
                                .execute_script("[douyu-crptext]", FastString::from(job.script))
                                .map_err(|e| format!("js room script execute failed: {e}"))?;

                            let rid_js = serde_json::to_string(&job.rid)
                                .map_err(|e| format!("failed to stringify rid: {e}"))?;
                            let did_js = serde_json::to_string(&job.did)
                                .map_err(|e| format!("failed to stringify did: {e}"))?;
                            let call_expr = format!("ub98484234({rid_js},{did_js},{});", job.ts);

                            let js_result = runtime
                                .execute_script("[douyu-call]", FastString::from(call_expr))
                                .map_err(|e| format!("js sign call failed: {e}"))?;

                            let params = {
                                let scope = &mut runtime.handle_scope();
                                let result = js_result.open(scope);
                                result.to_rust_string_lossy(scope)
                            };

                            Ok(params)
                        })();

                        let _ = job.respond_to.send(response);
                    }
                })
                .map_err(|e| format!("Failed to spawn douyu signer worker thread: {e}"))?;

            Ok(Self { tx })
        }

        async fn sign(
            &self,
            script: String,
            rid: String,
            did: String,
            ts: i64,
        ) -> Result<String, String> {
            let (tx, rx) = oneshot::channel();
            self.tx
                .send(SignJob {
                    script,
                    rid,
                    did,
                    ts,
                    respond_to: tx,
                })
                .map_err(|e| format!("Failed to send douyu sign job: {e}"))?;

            rx.await
                .map_err(|e| format!("Failed to receive douyu sign response: {e}"))?
        }
    }

    static SIGNER: OnceLock<Result<LegacyDouyuSignerWorker, String>> = OnceLock::new();

    fn get_worker() -> Result<&'static LegacyDouyuSignerWorker, String> {
        let result = SIGNER.get_or_init(LegacyDouyuSignerWorker::new);
        result.as_ref().map_err(|e| e.clone())
    }

    pub(super) async fn execute_js_sign(
        script: &str,
        rid: &str,
        did: &str,
        ts: i64,
    ) -> Result<String, String> {
        let worker = get_worker()?;
        worker
            .sign(script.to_string(), rid.to_string(), did.to_string(), ts)
            .await
    }
}

mod unified {
    use super::*;

    static SIGNER: OnceLock<Result<JsFunctionWorker, String>> = OnceLock::new();

    fn get_worker() -> Result<&'static JsFunctionWorker, String> {
        let result = SIGNER.get_or_init(|| {
            JsFunctionWorker::new(DOUYU_SIGN_UA, vec![("./cryptojs.min.js", CRYPTO_JS)])
        });
        result.as_ref().map_err(|e| e.clone())
    }

    pub(super) async fn execute_js_sign(
        script: &str,
        rid: &str,
        did: &str,
        ts: i64,
    ) -> Result<String, String> {
        let worker = get_worker()?;
        let crptext_js =
            serde_json::to_string(script).map_err(|e| format!("failed to stringify crptext: {e}"))?;
        let rid_js =
            serde_json::to_string(rid).map_err(|e| format!("failed to stringify rid: {e}"))?;
        let did_js =
            serde_json::to_string(did).map_err(|e| format!("failed to stringify did: {e}"))?;

        // Keep crptext isolated per call; return value must be a string for JsFunctionWorker.
        let expr = format!(
            "(() => {{\n  const __crptext = {crptext_js};\n  const __rid = {rid_js};\n  const __did = {did_js};\n  const __ts = {ts};\n  const __fn = new Function('__rid', '__did', '__ts', __crptext + '; return ub98484234(__rid,__did,__ts);');\n  return __fn(__rid, __did, __ts);\n}})()"
        );

        worker.eval_string(expr).await
    }
}

pub async fn execute_js_sign(script: &str, rid: &str, did: &str, ts: i64) -> Result<String, String> {
    match SIGNER_MODE {
        SignerMode::Unified => unified::execute_js_sign(script, rid, did, ts).await,
        SignerMode::Legacy => legacy::execute_js_sign(script, rid, did, ts).await,
    }
}
