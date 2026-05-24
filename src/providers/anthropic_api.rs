use serde::Deserialize;
use super::ProviderData;

#[derive(Debug, Deserialize)]
struct ApiUsageResponse {
    input_tokens: Option<u64>,
    tokens_used: Option<u64>,
    tokens_limit: Option<u64>,
    input_tokens_limit: Option<u64>,
    credits_used: Option<f64>,
    credits_limit: Option<f64>,
    reset_at: Option<String>,
}

pub async fn fetch_anthropic_api(api_key: &str) -> ProviderData {
    if api_key.is_empty() {
        return ProviderData {
            id: "anthropic-api".to_string(),
            name: "Anthropic API".to_string(),
            window_label: None,
            utilization: 0.0,
            reset_at: None,
            pace_info: None,
            used_credits: None,
            limit_credits: None,
            meta: None,
            error: Some("No API key configured".to_string()),
            tokens_used: None,
            cost_usd: None,
        };
    }

    let client = reqwest::Client::new();
    let result = client
        .get("https://api.anthropic.com/v1/usage")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .send()
        .await;

    match result {
        Err(e) => ProviderData {
            id: "anthropic-api".to_string(),
            name: "Anthropic API".to_string(),
            window_label: None,
            utilization: 0.0,
            reset_at: None,
            pace_info: None,
            used_credits: None,
            limit_credits: None,
            meta: None,
            error: Some(format!("Network error: {e}")),
            tokens_used: None,
            cost_usd: None,
        },
        Ok(resp) => {
            let status = resp.status();
            if !status.is_success() {
                return ProviderData {
                    id: "anthropic-api".to_string(),
                    name: "Anthropic API".to_string(),
                    window_label: None,
                    utilization: 0.0,
                    reset_at: None,
                    pace_info: None,
                    used_credits: None,
                    limit_credits: None,
                    meta: None,
                    error: Some(format!("API error ({})", status.as_u16())),
                    tokens_used: None,
                    cost_usd: None,
                };
            }
            match resp.json::<ApiUsageResponse>().await {
                Err(_) => ProviderData {
                    id: "anthropic-api".to_string(),
                    name: "Anthropic API".to_string(),
                    window_label: None,
                    utilization: 0.0,
                    reset_at: None,
                    pace_info: None,
                    used_credits: None,
                    limit_credits: None,
                    meta: None,
                    error: Some("Parse error".to_string()),
                    tokens_used: None,
                    cost_usd: None,
                },
                Ok(data) => {
                    let used = data.input_tokens.or(data.tokens_used).unwrap_or(0);
                    let limit = data.input_tokens_limit.or(data.tokens_limit).unwrap_or(0);
                    let utilization = if limit > 0 {
                        (used as f64 / limit as f64 * 100.0) as f32
                    } else {
                        0.0
                    };
                    let reset_at = data.reset_at
                        .as_deref()
                        .and_then(|s| s.parse().ok());
                    ProviderData {
                        id: "anthropic-api".to_string(),
                        name: "Anthropic API".to_string(),
                        window_label: None,
                        utilization,
                        reset_at,
                        pace_info: None,
                        used_credits: data.credits_used,
                        limit_credits: data.credits_limit,
                        meta: None,
                        error: None,
                        tokens_used: Some(used),
                        cost_usd: None,
                    }
                }
            }
        }
    }
}
