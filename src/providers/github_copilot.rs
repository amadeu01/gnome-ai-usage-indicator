use super::ProviderData;

/// GitHub Copilot Business usage via org-level API.
///
/// Requires a fine-grained PAT with:
///   - Resource owner: the Copilot org (e.g. SpexAI)
///   - "GitHub Copilot Business" permission → Read
///   - "Members" permission → Read
///
/// Endpoint: `GET /orgs/{org}/members/{username}/copilot`
pub async fn fetch_github_copilot(config_token: &str, copilot_org: &str) -> ProviderData {
    if copilot_org.is_empty() {
        return super::error_entry(
            "github-copilot",
            "GitHub Copilot",
            "Set copilot_org in config",
        );
    }

    let token = if !config_token.is_empty() {
        config_token.to_string()
    } else {
        match tokio::process::Command::new("gh")
            .args(["auth", "token"])
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                let t = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if t.is_empty() {
                    return super::error_entry(
                        "github-copilot",
                        "GitHub Copilot",
                        "No GitHub token",
                    );
                }
                t
            }
            _ => {
                return super::error_entry(
                    "github-copilot",
                    "GitHub Copilot",
                    "No GitHub token",
                );
            }
        }
    };

    // Resolve username from /user
    let client = reqwest::Client::new();
    let username = match resolve_username(&client, &token).await {
        Some(u) => u,
        None => {
            return super::error_entry(
                "github-copilot",
                "GitHub Copilot",
                "Could not resolve GitHub username",
            );
        }
    };

    let url = format!("https://api.github.com/orgs/{copilot_org}/members/{username}/copilot");
    let result = client
        .get(&url)
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "ai-usage-indicator")
        .send()
        .await;

    match result {
        Ok(resp) if resp.status().is_success() => {
            match resp.json::<serde_json::Value>().await {
                Ok(json) => parse_member_usage(&json, copilot_org),
                Err(_) => super::error_entry(
                    "github-copilot",
                    "GitHub Copilot",
                    "Parse error",
                ),
            }
        }
        Ok(resp) => {
            let status = resp.status().as_u16();
            let msg = if status == 403 {
                "Token lacks Copilot Business + Members org permissions"
            } else if status == 404 {
                "Org or member not found — check copilot_org"
            } else {
                return super::error_entry(
                    "github-copilot",
                    "GitHub Copilot",
                    &format!("HTTP {status}"),
                );
            };
            super::error_entry("github-copilot", "GitHub Copilot", msg)
        }
        Err(e) => super::error_entry(
            "github-copilot",
            "GitHub Copilot",
            &format!("Network error: {e}"),
        ),
    }
}

async fn resolve_username(client: &reqwest::Client, token: &str) -> Option<String> {
    let result = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {token}"))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "ai-usage-indicator")
        .send()
        .await
        .ok()?;

    let user: serde_json::Value = result.json().await.ok()?;
    user.get("login")?.as_str().map(String::from)
}

fn parse_member_usage(json: &serde_json::Value, org: &str) -> ProviderData {
    // Response shape (from GET /orgs/{org}/members/{username}/copilot):
    // { "seat_created_at": "...", "plan_type": "...", "usage": { ... } }
    // or: { "usage_percentage": N, "reset_date": "..." }
    let pct = json
        .get("usage_percentage")
        .or_else(|| json.get("usage").and_then(|u| u.get("percentage")))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0)
        .clamp(0.0, 100.0);

    let reset_date: Option<String> = json
        .get("reset_date")
        .or_else(|| json.get("usage").and_then(|u| u.get("reset_date")))
        .and_then(|v| v.as_str())
        .map(String::from);

    ProviderData {
        id: "github-copilot".to_string(),
        name: "GitHub Copilot".to_string(),
        utilization: pct as f32,
        reset_at: reset_date.as_deref().and_then(|s| s.parse().ok()),
        meta: if pct > 0.0 {
            Some(format!("{:.0}% used — org: {org}", pct))
        } else {
            None
        },
        ..Default::default()
    }
}
