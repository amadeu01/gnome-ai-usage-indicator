use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use zbus::{interface, SignalContext};
use crate::config::Config;
use crate::providers::ProviderData;

pub const BUS_NAME: &str = "io.github.amadeu01.AiUsageIndicator";
pub const OBJECT_PATH: &str = "/io/github/amadeu01/AiUsageIndicator";

pub struct AiUsageIndicatorInterface {
    pub data: Arc<Mutex<Vec<ProviderData>>>,
    pub refresh_notify: Arc<Notify>,
    pub config: Arc<Mutex<Config>>,
}

#[interface(name = "io.github.amadeu01.AiUsageIndicator")]
impl AiUsageIndicatorInterface {
    async fn get_provider_data(&self) -> String {
        let data = self.data.lock().await;
        serde_json::to_string(&*data).unwrap_or_else(|_| "[]".to_string())
    }

    async fn refresh(&self) {
        self.refresh_notify.notify_one();
    }

    async fn get_config(&self) -> String {
        let cfg = self.config.lock().await;
        let sanitized = serde_json::json!({
            "enabledProviders": cfg.enabled_providers,
            "pollIntervalSecs": cfg.poll_interval_secs,
            "ollamaHost": cfg.ollama_host,
            "anthropicApiKeyPresent": !cfg.anthropic_api_key.is_empty(),
        });
        serde_json::to_string(&sanitized).unwrap_or_else(|_| "{}".to_string())
    }

    #[zbus(signal)]
    pub async fn data_updated(signal_ctxt: &SignalContext<'_>, json: &str) -> zbus::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn provider_data_serializes_to_json() {
        let data = vec![ProviderData {
            id: "claude-code-5h".to_string(),
            name: "Claude Code".to_string(),
            window_label: Some("Session".to_string()),
            utilization: 42.5,
            reset_at: Some(Utc::now()),
            pace_info: None,
            used_credits: None,
            limit_credits: None,
            meta: None,
            error: None,
            tokens_used: Some(12345),
            cost_usd: Some(0.15),
        }];

        let json = serde_json::to_string(&data).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0]["id"], "claude-code-5h");
        assert_eq!(parsed[0]["utilization"], 42.5);
        assert_eq!(parsed[0]["windowLabel"], "Session");
        assert_eq!(parsed[0]["tokensUsed"], 12345);
    }

    #[test]
    fn empty_data_serializes_to_array() {
        let data: Vec<ProviderData> = vec![];
        let json = serde_json::to_string(&data).unwrap();
        assert_eq!(json, "[]");
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_empty());
    }
}
