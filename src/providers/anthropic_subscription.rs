use chrono::Utc;
use serde::Deserialize;
use sha2::{Sha256, Digest};
use std::path::PathBuf;
use std::time::UNIX_EPOCH;
use super::ProviderData;
use crate::config::home_dir;

const API_URL: &str = "https://api.anthropic.com/api/oauth/usage";
const BETA_HEADER: &str = "oauth-2025-04-20";
const USER_AGENT: &str = "claude-code/2.1.34";
const CACHE_MAX_AGE_SECS: u64 = 60;
const CONFIG_DIR: &str = ".claude";

#[derive(Debug, Deserialize)]
struct CredentialsFile {
    #[serde(rename = "claudeAiOauth")]
    claude_ai_oauth: OauthEntry,
}

#[derive(Debug, Deserialize)]
struct OauthEntry {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "expiresAt")]
    expires_at: i64,
}

#[derive(Debug, Deserialize)]
struct UsageWindow {
    utilization: Option<f64>,
    resets_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExtraUsage {
    is_enabled: bool,
    monthly_limit: Option<f64>,
    used_credits: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct OauthUsageResponse {
    five_hour: Option<UsageWindow>,
    seven_day: Option<UsageWindow>,
    extra_usage: Option<ExtraUsage>,
}

fn credentials_path() -> PathBuf {
    home_dir().join(CONFIG_DIR).join(".credentials.json")
}

fn cache_path() -> PathBuf {
    let config_dir = home_dir().join(CONFIG_DIR);
    let hash = hex::encode(&Sha256::digest(config_dir.to_string_lossy().as_bytes())[..4]);
    PathBuf::from(format!("/tmp/claude/statusline-usage-cache-{hash}.json"))
}

fn read_token() -> Result<String, String> {
    let path = credentials_path();
    let contents = std::fs::read_to_string(&path)
        .map_err(|_| "No credentials file — run `claude login`".to_string())?;

    let creds: CredentialsFile = serde_json::from_str(&contents)
        .map_err(|e| format!("Credentials parse error: {e}"))?;

    let now_ms = Utc::now().timestamp_millis();
    if creds.claude_ai_oauth.expires_at < now_ms {
        return Err("Token expired — run `claude login` to refresh".to_string());
    }

    Ok(creds.claude_ai_oauth.access_token)
}

fn read_cache() -> Option<OauthUsageResponse> {
    let path = cache_path();
    let meta = std::fs::metadata(&path).ok()?;
    let mtime = meta.modified().ok()?;
    let age_secs = UNIX_EPOCH.elapsed().ok()?.as_secs()
        .saturating_sub(mtime.duration_since(UNIX_EPOCH).ok()?.as_secs());

    if age_secs >= CACHE_MAX_AGE_SECS {
        return None;
    }

    let contents = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&contents).ok()
}

async fn fetch_usage(token: &str) -> Result<OauthUsageResponse, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(API_URL)
        .header("Authorization", format!("Bearer {token}"))
        .header("anthropic-beta", BETA_HEADER)
        .header("Accept", "application/json")
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    let status = resp.status();
    if !status.is_success() {
        return Err(format!("API error ({})", status.as_u16()));
    }

    let body = resp.text().await.map_err(|e| format!("Read error: {e}"))?;

    let data: OauthUsageResponse =
        serde_json::from_str(&body).map_err(|e| format!("Parse error: {e}"))?;

    // Write to cache
    let cache = cache_path();
    if let Some(parent) = cache.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&cache, &body);

    Ok(data)
}

fn window_to_entry(data: &UsageWindow, id_suffix: &str, label: &str) -> ProviderData {
    let utilization = data.utilization.unwrap_or(0.0) as f32;
    let reset_at = data
        .resets_at
        .as_deref()
        .and_then(|s| s.parse().ok());

    ProviderData {
        id: format!("anthropic-sub-{id_suffix}"),
        name: "Anthropic Subscription".to_string(),
        window_label: Some(label.to_string()),
        utilization,
        reset_at,
        ..Default::default()
    }
}

pub async fn fetch_anthropic_subscription() -> Vec<ProviderData> {
    let token = match read_token() {
        Ok(t) => t,
        Err(e) => return vec![super::error_entry("anthropic-subscription", "Anthropic Subscription", &e)],
    };

    // Try cache first, fall back to API
    let usage_data = match read_cache() {
        Some(data) => data,
        None => match fetch_usage(&token).await {
            Ok(data) => data,
            Err(e) => return vec![super::error_entry("anthropic-subscription", "Anthropic Subscription", &e)],
        },
    };

    let mut results = Vec::new();

    if let Some(ref w) = usage_data.five_hour {
        results.push(window_to_entry(w, "5h", "Session"));
    }

    if let Some(ref w) = usage_data.seven_day {
        results.push(window_to_entry(w, "7d", "Weekly"));
    }

    if let Some(ref extra) = usage_data.extra_usage {
        if extra.is_enabled && extra.used_credits.unwrap_or(0.0) > 0.0 {
            results.push(ProviderData {
                id: "anthropic-sub-extra".to_string(),
                name: "Anthropic Subscription".to_string(),
                window_label: Some("Extra usage".to_string()),
                utilization: 0.0,
                used_credits: extra.used_credits,
                limit_credits: extra.monthly_limit,
                ..Default::default()
            });
        }
    }


    if results.is_empty() {
        results.push(super::error_entry(
            "anthropic-subscription",
            "Anthropic Subscription",
            "No usage windows in API response",
        ));
    }

    results
}
