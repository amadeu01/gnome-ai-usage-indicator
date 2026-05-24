use chrono::{DateTime, Utc};
use crate::config::Config;

pub mod claude_code;
pub mod ollama;

#[derive(Debug, Clone)]
pub struct PaceInfo {
    pub label: String,
    pub ahead: bool,
}

#[derive(Debug, Clone)]
pub struct ProviderData {
    pub id: String,
    pub name: String,
    pub window_label: Option<String>,
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
    let mut results = Vec::new();

    let fetch_claude = config.enabled_providers.contains(&"claude-code".to_string());
    let fetch_ollama = config.enabled_providers.contains(&"ollama".to_string());

    let (claude_result, ollama_result) = tokio::join!(
        async {
            if fetch_claude {
                Some(claude_code::fetch_claude_code().await)
            } else {
                None
            }
        },
        async {
            if fetch_ollama {
                Some(ollama::fetch_ollama(&config.ollama_host).await)
            } else {
                None
            }
        }
    );

    if let Some(entries) = claude_result {
        results.extend(entries);
    }
    if let Some(entry) = ollama_result {
        results.push(entry);
    }

    results
}
