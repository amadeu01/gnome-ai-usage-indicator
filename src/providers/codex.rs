use serde::Deserialize;
use std::path::PathBuf;
use super::ProviderData;

#[derive(Debug, Deserialize)]
struct CodexUsage {
    usage_pct: Option<f64>,
    tokens_used: Option<u64>,
    tokens_limit: Option<u64>,
    credits_used: Option<f64>,
    credits_limit: Option<f64>,
    resets_at: Option<String>,
    reset_at: Option<String>,
}

fn codex_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".codex")
}

fn error_entry(msg: &str) -> ProviderData {
    ProviderData {
        id: "codex".to_string(),
        name: "Codex".to_string(),
        window_label: None,
        utilization: 0.0,
        reset_at: None,
        pace_info: None,
        used_credits: None,
        limit_credits: None,
        meta: None,
        error: Some(msg.to_string()),
        tokens_used: None,
        cost_usd: None,
    }
}

pub fn fetch_codex() -> ProviderData {
    let dir = codex_dir();
    if !dir.exists() {
        return error_entry("Not installed");
    }

    let candidates = ["usage.json", "state.json", "config.json"];
    for name in &candidates {
        let path = dir.join(name);
        if !path.exists() {
            continue;
        }
        let Ok(contents) = std::fs::read_to_string(&path) else { continue };
        let Ok(data) = serde_json::from_str::<CodexUsage>(&contents) else { continue };

        if let Some(pct) = data.usage_pct {
            let reset_at = data.resets_at.as_deref()
                .and_then(|s| s.parse().ok());
            return ProviderData {
                id: "codex".to_string(),
                name: "Codex".to_string(),
                window_label: None,
                utilization: pct as f32,
                reset_at,
                pace_info: None,
                used_credits: data.credits_used,
                limit_credits: data.credits_limit,
                meta: None,
                error: None,
                tokens_used: None,
                cost_usd: None,
            };
        }

        if let (Some(used), Some(limit)) = (data.tokens_used, data.tokens_limit) {
            let utilization = if limit > 0 {
                (used as f64 / limit as f64 * 100.0) as f32
            } else {
                0.0
            };
            let reset_at = data.reset_at.as_deref()
                .and_then(|s| s.parse().ok());
            return ProviderData {
                id: "codex".to_string(),
                name: "Codex".to_string(),
                window_label: None,
                utilization,
                reset_at,
                pace_info: None,
                used_credits: None,
                limit_credits: None,
                meta: None,
                error: None,
                tokens_used: Some(used),
                cost_usd: None,
            };
        }
    }

    error_entry("No usage data")
}
