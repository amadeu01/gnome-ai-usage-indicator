use serde::Deserialize;
use super::ProviderData;

#[derive(Debug, Deserialize)]
struct BalanceInfo {
    currency: String,
    total_balance: String,
    granted_balance: String,
    topped_up_balance: String,
}

#[derive(Debug, Deserialize)]
struct BalanceResponse {
    is_available: bool,
    balance_infos: Vec<BalanceInfo>,
}

pub async fn fetch_deepseek(api_key: &str) -> ProviderData {
    if api_key.is_empty() {
        return super::error_entry("deepseek", "DeepSeek", "No API key configured");
    }

    let client = reqwest::Client::new();
    let result = client
        .get("https://api.deepseek.com/v1/user/balance")
        .header("Authorization", format!("Bearer {api_key}"))
        .send()
        .await;

    match result {
        Err(e) => super::error_entry("deepseek", "DeepSeek", &format!("Network error: {e}")),
        Ok(resp) => {
            let status = resp.status();
            if !status.is_success() {
                return super::error_entry(
                    "deepseek",
                    "DeepSeek",
                    &format!("API error ({})", status.as_u16()),
                );
            }
            match resp.json::<BalanceResponse>().await {
                Err(_) => super::error_entry("deepseek", "DeepSeek", "Parse error"),
                Ok(data) => {
                    let meta = if data.is_available && !data.balance_infos.is_empty() {
                        let b = &data.balance_infos[0];
                        let total: f64 = b.total_balance.parse().unwrap_or(0.0);
                        let granted: f64 = b.granted_balance.parse().unwrap_or(0.0);
                        let topped: f64 = b.topped_up_balance.parse().unwrap_or(0.0);
                        Some(format!(
                            "Balance: {:.2} {} (granted {:.2}, topped up {:.2})",
                            total, b.currency, granted, topped
                        ))
                    } else {
                        Some("Balance unavailable".to_string())
                    };

                    ProviderData {
                        id: "deepseek".to_string(),
                        name: "DeepSeek".to_string(),
                        utilization: 0.0,
                        meta,
                        ..Default::default()
                    }
                }
            }
        }
    }
}
