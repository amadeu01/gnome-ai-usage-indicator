use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::config::Config;

pub mod anthropic_api;
pub mod claude_code;
pub mod codex;
pub mod ollama;

#[derive(Debug, Clone, Serialize)]
pub struct PaceInfo {
    pub label: String,
    pub ahead: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderData {
    pub id: String,
    pub name: String,
    pub window_label: Option<String>,
    /// 0–100 utilization percentage
    pub utilization: f32,
    pub reset_at: Option<DateTime<Utc>>,
    pub pace_info: Option<PaceInfo>,
    pub used_credits: Option<f64>,
    pub limit_credits: Option<f64>,
    pub meta: Option<String>,
    pub error: Option<String>,
    pub tokens_used: Option<u64>,
    pub cost_usd: Option<f64>,
}

pub async fn fetch_all(config: &Config) -> Vec<ProviderData> {
    let fetch_claude = config.enabled_providers.contains(&"claude-code".to_string());
    let fetch_ollama = config.enabled_providers.contains(&"ollama".to_string());
    let fetch_anthropic = config.enabled_providers.contains(&"anthropic-api".to_string());
    let fetch_codex = config.enabled_providers.contains(&"codex".to_string());

    let (claude_result, ollama_result, anthropic_result, codex_result) = tokio::join!(
        async {
            if fetch_claude { Some(claude_code::fetch_claude_code().await) } else { None }
        },
        async {
            if fetch_ollama { Some(ollama::fetch_ollama(&config.ollama_host).await) } else { None }
        },
        async {
            if fetch_anthropic {
                Some(anthropic_api::fetch_anthropic_api(&config.anthropic_api_key).await)
            } else {
                None
            }
        },
        async {
            if fetch_codex { Some(codex::fetch_codex()) } else { None }
        },
    );

    let mut results = Vec::new();
    if let Some(entries) = claude_result { results.extend(entries); }
    if let Some(entry) = ollama_result { results.push(entry); }
    if let Some(entry) = anthropic_result { results.push(entry); }
    if let Some(entry) = codex_result { results.push(entry); }
    results
}
