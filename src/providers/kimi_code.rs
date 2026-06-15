use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;
use super::ProviderData;
use crate::config::home_dir;

const CONFIG_DIR: &str = ".kimi-code";

#[derive(Debug, Deserialize)]
struct CredentialsFile {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct KimiConfig {
    default_model: Option<String>,
}

fn credentials_path() -> PathBuf {
    home_dir().join(CONFIG_DIR).join("credentials").join("kimi-code.json")
}

fn config_path() -> PathBuf {
    home_dir().join(CONFIG_DIR).join("config.toml")
}

fn session_index_path() -> PathBuf {
    home_dir().join(CONFIG_DIR).join("session_index.jsonl")
}

/// Decode the `exp` claim from a JWT access token. Returns None if decoding fails.
fn jwt_exp(token: &str) -> Option<i64> {
    let payload_b64 = token.split('.').nth(1)?;
    let bytes = URL_SAFE_NO_PAD.decode(payload_b64).ok()?;
    let claims: Value = serde_json::from_slice(&bytes).ok()?;
    claims.get("exp")?.as_i64()
}

/// Read the active model name from Kimi Code config.
fn read_model() -> Option<String> {
    let contents = std::fs::read_to_string(config_path()).ok()?;
    let cfg: KimiConfig = toml::from_str(&contents).ok()?;
    cfg.default_model
}

/// Count sessions from session_index.jsonl.
fn count_sessions() -> usize {
    std::fs::read_to_string(session_index_path())
        .map(|s| s.lines().count())
        .unwrap_or(0)
}

/// Probe quota status with a minimal inference request (max_tokens=1).
/// Returns Some(100.0) if quota exhausted, None if probe inconclusive.
async fn probe_quota(token: &str) -> Option<f32> {
    let client = reqwest::Client::new();
    let body = json!({
        "model": "kimi-for-coding",
        "messages": [{"role": "user", "content": "1"}],
        "max_tokens": 1
    });

    let resp = client
        .post("https://api.kimi.com/coding/v1/chat/completions")
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .ok()?;

    if resp.status().as_u16() == 403 {
        let text = resp.text().await.ok()?;
        if text.contains("access_terminated_error") {
            return Some(100.0);
        }
    }
    None
}

pub async fn fetch_kimi_code() -> ProviderData {
    let path = credentials_path();
    if !path.exists() {
        return super::error_entry("kimi-code", "Kimi Code", "Not authenticated — no credentials file");
    }

    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => return super::error_entry("kimi-code", "Kimi Code", &format!("Cannot read credentials: {e}")),
    };

    let creds: CredentialsFile = match serde_json::from_str(&contents) {
        Ok(c) => c,
        Err(e) => return super::error_entry("kimi-code", "Kimi Code", &format!("Credentials parse error: {e}")),
    };

    let exp = match jwt_exp(&creds.access_token) {
        Some(e) => e,
        None => return super::error_entry("kimi-code", "Kimi Code", "Cannot decode token — run `kimi login`"),
    };

    let now_secs = Utc::now().timestamp();
    if exp <= now_secs {
        return super::error_entry("kimi-code", "Kimi Code", "Token expired — run `kimi login` to refresh");
    }

    // Check if quota is exhausted via lightweight probe
    let utilization = probe_quota(&creds.access_token).await.unwrap_or(0.0);

    let model = read_model().unwrap_or_else(|| "unknown".to_string());
    let session_count = count_sessions();
    let meta = if utilization > 0.0 {
        None
    } else {
        Some(format!("Model: {} · {} sessions", model, session_count))
    };

    ProviderData {
        id: "kimi-code".to_string(),
        name: "Kimi Code".to_string(),
        utilization,
        meta,
        ..Default::default()
    }
}
