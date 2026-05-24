use serde::Deserialize;
use std::time::Duration;
use super::ProviderData;

#[derive(Debug, Deserialize)]
struct OllamaModel {
    name: String,
    size_vram: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct OllamaPsResponse {
    models: Vec<OllamaModel>,
}

pub async fn fetch_ollama(host: &str) -> ProviderData {
    let url = format!("{}/api/ps", host.trim_end_matches('/'));
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default();

    match client.get(&url).send().await {
        Err(_) => ProviderData {
            id: "ollama".to_string(),
            name: "Ollama".to_string(),
            window_label: None,
            utilization: 0.0,
            reset_at: None,
            pace_info: None,
            used_credits: None,
            limit_credits: None,
            meta: None,
            error: Some("Not running".to_string()),
            tokens_used: None,
            cost_usd: None,
        },
        Ok(resp) => {
            match resp.json::<OllamaPsResponse>().await {
                Err(_) => ProviderData {
                    id: "ollama".to_string(),
                    name: "Ollama".to_string(),
                    window_label: None,
                    utilization: 0.0,
                    reset_at: None,
                    pace_info: None,
                    used_credits: None,
                    limit_credits: None,
                    meta: None,
                    error: Some("Invalid response".to_string()),
                    tokens_used: None,
                    cost_usd: None,
                },
                Ok(ps) => {
                    let meta = if ps.models.is_empty() {
                        Some("No models loaded".to_string())
                    } else {
                        let names: Vec<String> = ps.models.iter().map(|m| {
                            if let Some(vram) = m.size_vram {
                                let gb = vram as f64 / 1_000_000_000.0;
                                format!("{} ({:.1} GB)", m.name, gb)
                            } else {
                                m.name.clone()
                            }
                        }).collect();
                        Some(names.join(", "))
                    };
                    ProviderData {
                        id: "ollama".to_string(),
                        name: "Ollama".to_string(),
                        window_label: None,
                        utilization: 0.0,
                        reset_at: None,
                        pace_info: None,
                        used_credits: None,
                        limit_credits: None,
                        meta,
                        error: None,
                        tokens_used: None,
                        cost_usd: None,
                    }
                }
            }
        }
    }
}
