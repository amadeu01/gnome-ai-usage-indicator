use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::path::PathBuf;
use super::{PaceInfo, ProviderData};
use crate::config::home_dir;

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
    utilization: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct CostData {
    total_tokens: Option<u64>,
    total_usd: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct UsageCache {
    five_hour: Option<UsageWindow>,
    seven_day: Option<UsageWindow>,
    extra_usage: Option<ExtraUsage>,
    cost: Option<CostData>,
}

fn cache_path() -> PathBuf {
    let mut p = home_dir();
    p.push(".claude/statusline-usage-cache.json");
    p
}

fn compute_pace(utilization: f64, resets_at: &str) -> Option<PaceInfo> {
    let reset: DateTime<Utc> = resets_at.parse().ok()?;
    let now = Utc::now();
    let window_secs = 7.0 * 86400.0;
    let elapsed_secs = (window_secs - (reset - now).num_seconds() as f64).max(0.0);
    let elapsed_fraction = (elapsed_secs / window_secs).clamp(0.0, 1.0);
    if elapsed_fraction < 1e-6 {
        return None;
    }
    let used_fraction = (utilization / 100.0).clamp(0.0, 1.0);
    let pace_ratio = used_fraction / elapsed_fraction;
    let pct_diff = ((pace_ratio - 1.0) * 100.0).round() as i64;
    let (label, ahead) = if pace_ratio > 1.0 {
        (format!("Pace: Ahead (+{}%)", pct_diff), true)
    } else if pace_ratio < 0.8 {
        (format!("Pace: Behind ({}%)", pct_diff), false)
    } else {
        ("Pace: On track".to_string(), true)
    };
    Some(PaceInfo { label, ahead })
}

pub async fn fetch_claude_code() -> Vec<ProviderData> {
    let path = cache_path();
    let contents = match tokio::fs::read_to_string(&path).await {
        Ok(c) => c,
        Err(_) => {
            return vec![super::error_entry("claude-code", "Claude Code", "No data — is Claude Code installed?")];
        }
    };
    let cache: UsageCache = match serde_json::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            return vec![super::error_entry("claude-code", "Claude Code", &format!("Parse error: {e}"))];
        }
    };

    let (tokens_used, cost_usd) = match &cache.cost {
        Some(c) => (c.total_tokens, c.total_usd),
        None => (None, None),
    };

    let mut results = Vec::new();

    if let Some(ref w) = cache.five_hour {
        let util = w.utilization.unwrap_or(0.0) as f32;
        let reset_at = w.resets_at.as_deref()
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());
        results.push(ProviderData {
            id: "claude-code-5h".to_string(),
            name: "Claude Code".to_string(),
            window_label: Some("Session".to_string()),
            utilization: util,
            reset_at,
            ..Default::default()
        });
    }

    if let Some(ref w) = cache.seven_day {
        let util = w.utilization.unwrap_or(0.0);
        let reset_at = w.resets_at.as_deref()
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());
        let pace_info = w.resets_at.as_deref()
            .and_then(|s| compute_pace(util, s));
        results.push(ProviderData {
            id: "claude-code-7d".to_string(),
            name: "Claude Code".to_string(),
            window_label: Some("Weekly".to_string()),
            utilization: util as f32,
            reset_at,
            pace_info,
            tokens_used,
            cost_usd,
            ..Default::default()
        });
    }

    if let Some(ref extra) = cache.extra_usage {
        if extra.is_enabled {
            results.push(ProviderData {
                id: "claude-code-extra".to_string(),
                name: "Claude Code".to_string(),
                window_label: Some("Extra usage".to_string()),
                utilization: extra.utilization.unwrap_or(0.0) as f32,
                used_credits: extra.used_credits,
                limit_credits: extra.monthly_limit,
                ..Default::default()
            });
        }
    }

    if results.is_empty() {
        results.push(super::error_entry("claude-code", "Claude Code", "No usage windows found"));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::error_entry;

    #[test]
    fn pace_ahead() {
        // 3 days elapsed of 7-day window, used 60% → pace_ratio = 0.6/0.43 ≈ 1.4 → ahead
        let future = chrono::Utc::now() + chrono::Duration::days(4);
        let resets_at = future.to_rfc3339();
        let info = compute_pace(60.0, &resets_at).unwrap();
        assert!(info.ahead);
        assert!(info.label.contains("Ahead"));
    }

    #[test]
    fn pace_behind() {
        // 4 days elapsed of 7-day window, only 20% used → behind pace
        let future = chrono::Utc::now() + chrono::Duration::days(3);
        let resets_at = future.to_rfc3339();
        let info = compute_pace(20.0, &resets_at).unwrap();
        assert!(!info.ahead);
        assert!(info.label.contains("Behind"));
    }

    #[test]
    fn parse_error_entry_has_error_field() {
        let e = error_entry("claude-code", "Claude Code", "test error");
        assert!(e.error.is_some());
        assert_eq!(e.error.unwrap(), "test error");
    }
}
