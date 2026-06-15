## 1. Dependencies

- [ ] 1.1 Add `sha2 = "0.10"` to `Cargo.toml` under `[dependencies]`
- [ ] 1.2 Add `hex = "0.4"` to `Cargo.toml` under `[dependencies]`

## 2. Rust: Anthropic Subscription Provider

- [ ] 2.1 Create `src/providers/anthropic_subscription.rs`
- [ ] 2.2 Define deserialization structs:
  - `CredentialsFile { claudeAiOauth: OauthEntry }`
  - `OauthEntry { accessToken: String, expiresAt: i64, subscriptionType: String }`
  - `OauthUsageResponse { five_hour: Option<UsageWindow>, seven_day: Option<UsageWindow>, extra_usage: Option<ExtraUsage> }`
  - `UsageWindow { utilization: Option<f64>, resets_at: Option<String> }`
  - `ExtraUsage { is_enabled: bool, monthly_limit: Option<f64>, used_credits: Option<f64>, utilization: Option<f64> }`
- [ ] 2.3 Implement `fn credentials_path() -> PathBuf` — `home_dir().join(".claude/.credentials.json")`
- [ ] 2.4 Implement `fn cache_path() -> PathBuf`:
  - Compute SHA256 of `home_dir().join(".claude")` → first 4 bytes as hex → `format!("/tmp/claude/statusline-usage-cache-{hash}.json")`
- [ ] 2.5 Implement `fn read_token() -> Result<String>` — read credentials file, parse JSON, check `expiresAt` against `Utc::now()`, return access token or error
- [ ] 2.6 Implement `fn read_cache() -> Option<OauthUsageResponse>` — read cache file if exists and mtime < 60s old, parse JSON
- [ ] 2.7 Implement `async fn fetch_usage(token: &str) -> Result<OauthUsageResponse>`:
  - GET `https://api.anthropic.com/api/oauth/usage`
  - Headers: `Authorization: Bearer <token>`, `anthropic-beta: oauth-2025-04-20`, `User-Agent: claude-code/2.1.34`
  - Write response to cache file on success
- [ ] 2.8 Implement `pub async fn fetch_anthropic_subscription() -> Vec<ProviderData>`:
  - Call `read_token()` → error entry on failure
  - Call `read_cache()` → use if Some
  - Otherwise call `fetch_usage(&token)` → error entry on failure
  - Map `UsageWindow` → `ProviderData` with `window_label: "Session"` / `"Weekly"`, `id: "anthropic-sub-5h"` / `"anthropic-sub-7d"`
  - Map `ExtraUsage` → `ProviderData` with `window_label: "Extra usage"`, `id: "anthropic-sub-extra"`, `used_credits`, `limit_credits`
  - Return empty vec if no windows present

## 3. Rust: Provider Registration

- [ ] 3.1 Add `pub mod anthropic_subscription;` to `src/providers/mod.rs`
- [ ] 3.2 Add `let fetch_anthropic_sub = config.enabled_providers.contains(&"anthropic-subscription".to_string());` in `fetch_all`
- [ ] 3.3 Add `anthropic_sub_result` arm to `tokio::join!` — call `anthropic_subscription::fetch_anthropic_subscription().await`
- [ ] 3.4 Add `if let Some(entries) = anthropic_sub_result { results.extend(entries); }` (note: returns `Vec`, use `extend` not `push`)

## 4. GNOME Extension: TypeScript Types

- [ ] 4.1 Add `ANTHROPIC_SUBSCRIPTION: 'anthropic-subscription'` to `PROVIDER_IDS` in `ts-src/src/types.ts`

## 5. GNOME Extension: Preferences UI

- [ ] 5.1 Add entry to `providers` array in `ts-src/src/prefs.ts`:
  - `id: PROVIDER_IDS.ANTHROPIC_SUBSCRIPTION`
  - `label: 'Anthropic Subscription'`
  - `subtitle: 'Fetches from api.anthropic.com (OAuth, auto-discovered key)'`

## 6. Build & Verify

- [ ] 6.1 Run `cargo check` — confirm no compile errors
- [ ] 6.2 Run `cargo test` — confirm existing tests pass
- [ ] 6.3 Verify TypeScript compiles in `ts-src/`
- [ ] 6.4 Test end-to-end with live credentials token
