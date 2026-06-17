use chrono::{DateTime, Utc};
use serde::Deserialize;
use super::ProviderData;

#[derive(Debug, Deserialize)]
struct CopilotUsage {
    /// Percentage of monthly allowance used (0–100)
    #[serde(alias = "usage_percentage", alias = "percentUsed")]
    percentage: Option<f64>,
    /// ISO-8601 reset date string
    #[serde(alias = "reset_date", alias = "resetDate")]
    reset_date: Option<String>,
    /// Some responses include a human-readable summary
    #[serde(alias = "summary")]
    summary: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CopilotResponse {
    /// Direct fields on the response
    #[serde(flatten)]
    usage: CopilotUsage,
    /// Nested: some endpoints wrap in a "seat" or "data" object
    #[serde(alias = "seat", alias = "data")]
    seat: Option<CopilotUsage>,
}

pub async fn fetch_github_copilot(token: &str) -> ProviderData {
    if token.is_empty() {
        return super::error_entry("github-copilot", "GitHub Copilot", "No GitHub token configured");
    }

    let client = reqwest::Client::new();

    // Try documented org-level endpoint first; also works for personal accounts
    // on some GitHub deployments via the user context.
    let result = client
        .get("https://api.github.com/copilot/usage")
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "ai-usage-indicator")
        .send()
        .await;

    match result {
        Err(e) => super::error_entry("github-copilot", "GitHub Copilot", &format!("Network error: {e}")),
        Ok(resp) => {
            let status = resp.status();
            if status == 404 {
                // Fall back to user-level endpoint
                return fetch_copilot_user(&client, token).await;
            }
            if !status.is_success() {
                return super::error_entry(
                    "github-copilot",
                    "GitHub Copilot",
                    &format!("API error ({})", status.as_u16()),
                );
            }
            parse_copilot_response(resp).await
        }
    }
}

async fn fetch_copilot_user(client: &reqwest::Client, token: &str) -> ProviderData {
    let result = client
        .get("https://api.github.com/user/copilot/usage")
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "ai-usage-indicator")
        .send()
        .await;

    match result {
        Err(e) => super::error_entry("github-copilot", "GitHub Copilot", &format!("Network error: {e}")),
        Ok(resp) => {
            let status = resp.status();
            if !status.is_success() {
                return super::error_entry(
                    "github-copilot",
                    "GitHub Copilot",
                    &format!("API error ({})", status.as_u16()),
                );
            }
            parse_copilot_response(resp).await
        }
    }
}

async fn parse_copilot_response(resp: reqwest::Response) -> ProviderData {
    match resp.json::<CopilotResponse>().await {
        Err(e) => super::error_entry("github-copilot", "GitHub Copilot", &format!("Parse error: {e}")),
        Ok(data) => {
            let usage = data.seat.unwrap_or(data.usage);
            let pct = usage.percentage.unwrap_or(0.0);
            let utilization = pct as f32;

            let reset_at: Option<DateTime<Utc>> = usage
                .reset_date
                .as_deref()
                .and_then(|s| s.parse().ok());

            let meta = match (usage.summary.as_deref(), &usage.reset_date) {
                (Some(summary), _) => Some(summary.to_string()),
                (None, Some(date)) => Some(format!("Resets {}", date)),
                (None, None) if utilization > 0.0 => Some(format!("{:.0}% used", pct)),
                _ => None,
            };

            ProviderData {
                id: "github-copilot".to_string(),
                name: "GitHub Copilot".to_string(),
                utilization,
                reset_at,
                meta,
                ..Default::default()
            }
        }
    }
}
