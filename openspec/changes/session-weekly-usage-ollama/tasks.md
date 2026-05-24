## 1. SQLite History Store

- [ ] 1.1 Add `rusqlite` dependency to `Cargo.toml`
- [ ] 1.2 Create `src/history/mod.rs` with `HistoryStore` struct
- [ ] 1.3 Implement `new(db_path: &Path) -> Result<Self>` — opens DB, creates table if not exists
- [ ] 1.4 Implement `insert_snapshot(&self, data: &ProviderData) -> Result<()>` — inserts session + weekly rows
- [ ] 1.5 Implement `query_daily_trend(&self, provider_id: &str, days: u32) -> Result<Vec<UsagePoint>>` — groups by day, last value per day
- [ ] 1.6 Implement `query_hourly_progress(&self, provider_id: &str, window_hours: u32) -> Result<Vec<UsagePoint>>` — all points in current window
- [ ] 1.7 Add `UsagePoint` struct: `timestamp: DateTime<Utc>`, `utilization: f32`, `tokens_used: Option<u64>`, `cost_usd: Option<f64>`
- [ ] 1.8 Run migrations on startup: `CREATE TABLE IF NOT EXISTS usage_snapshots ...`
- [ ] 1.9 Add `PRAGMA journal_mode=WAL` for better concurrent reads

## 2. Ollama Cloud Provider (Cookie Auth)

- [ ] 2.1 Update `src/providers/ollama.rs` — remove localhost polling code
- [ ] 2.2 Add `OllamaCloudProvider` struct with `cookie: String` field
- [ ] 2.3 Implement `fetch_ollama_cloud(cookie: &str) -> Result<Vec<ProviderData>>`
- [ ] 2.4 Use `reqwest::Client` with `Cookie` header set
- [ ] 2.5 Fetch `https://ollama.com/settings` HTML
- [ ] 2.6 Parse HTML with `scraper` crate (CSS selectors):
  - Plan badge: `.plan-badge` text (Free/Pro/Max)
  - Session usage: `[data-window="session"] .usage-percent`
  - Weekly usage: `[data-window="weekly"] .usage-percent`
  - Reset times: `[data-window="session"] .reset-time[data-time]` → parse `data-time` attribute
- [ ] 2.7 Handle errors: cookie expired, HTML structure changed, network error
- [ ] 2.8 Return `ProviderData` with `window_label: "Session"` and `"Weekly"`, `utilization`, `reset_at`, `meta: Some(plan_tier)`
- [ ] 2.9 Add tests with sample HTML fixtures

## 3. OpenAI Codex Provider (OAuth)

- [ ] 3.1 Create `src/providers/codex.rs`
- [ ] 3.2 Add `CodexOAuthProvider` struct with `auth_path: PathBuf`
- [ ] 3.3 Implement `read_oauth_token(auth_path: &Path) -> Result<String>` — parse `~/.codex/auth.json`, extract access token
- [ ] 3.4 Implement `fetch_codex_usage(token: &str) -> Result<Vec<ProviderData>>`
- [ ] 3.5 GET `https://chatgpt.com/backend-api/wham/usage` with `Authorization: Bearer <token>`
- [ ] 3.6 Parse JSON response:
  - Session window: `rate_limits.session.utilization`, `resets_at`
  - Weekly window: `rate_limits.weekly.utilization`, `resets_at`
  - Credits: `credits.balance`, `credits.limit`
- [ ] 3.7 Handle 401 (token expired) → return error with "Token expired" message
- [ ] 3.8 Return `ProviderData` with `window_label`, `utilization`, `reset_at`, `used_credits`, `limit_credits`
- [ ] 3.9 Add tests with sample JSON fixtures

## 4. Claude Code Provider (Minor Updates)

- [ ] 4.1 Verify `src/providers/claude_code.rs` still works with current cache format
- [ ] 4.2 Add `cost_usd` field to returned `ProviderData` if present in cache
- [ ] 4.3 Improve error messages (path not found, JSON parse error)

## 5. Provider Manager Refactor

- [ ] 5.1 Create `Provider` trait in `src/providers/mod.rs`:
  ```rust
  #[async_trait]
  pub trait Provider: Send + Sync {
      fn id(&self) -> &str;
      fn name(&self) -> &str;
      async fn fetch_usage(&self) -> Result<Vec<ProviderData>>;
  }
  ```
- [ ] 5.2 Implement `Provider` trait for `ClaudeCodeProvider`, `OllamaCloudProvider`, `CodexOAuthProvider`
- [ ] 5.3 Update `fetch_all(config: &Config)` to build provider list dynamically based on enabled providers
- [ ] 5.4 Run providers concurrently with `tokio::join!`
- [ ] 5.5 Flatten results into `Vec<ProviderData>`
- [ ] 5.6 Add error handling per provider (one failure doesn't stop others)

## 6. Config Extensions

- [ ] 6.1 Update `src/config.rs` with new fields:
  ```rust
  pub struct Config {
      pub providers: ProviderConfigs,
      pub history: HistoryConfig,
      ...
  }
  ```
- [ ] 6.2 Add `ProviderConfigs` struct:
  ```rust
  pub struct ProviderConfigs {
      pub claude_code: ClaudeCodeConfig,
      pub ollama: OllamaConfig,
      pub codex: CodexConfig,
  }
  ```
- [ ] 6.3 Add `OllamaConfig { enabled: bool, cookie: String }`
- [ ] 6.4 Add `CodexConfig { enabled: bool, auth_path: Option<PathBuf> }` (default: `~/.codex/auth.json`)
- [ ] 6.5 Add `HistoryConfig { enabled: bool, db_path: Option<PathBuf>, retention_days: u32 }`
- [ ] 6.6 Default `db_path` to `~/.local/share/ai-usage-indicator/history.db`
- [ ] 6.7 Default `retention_days` to 30
- [ ] 6.8 Update config file documentation with examples

## 7. Usage History Window (GTK4)

- [ ] 7.1 Create `src/ui/history_window.rs`
- [ ] 7.2 Add `HistoryWindow` struct with `ApplicationWindow`, `provider_selector`, `window_selector`, `graph_area`
- [ ] 7.3 Build UI with `gtk4::Builder` or programmatically:
  - Header bar with title
  - Provider dropdown (Claude Code, Ollama Cloud, OpenAI Codex)
  - Window dropdown (Session, Weekly)
  - `gtk4::Picture` widget for graph
  - Status label (last updated, data source)
- [ ] 7.4 Implement `new(app: &Application) -> Self`
- [ ] 7.5 Implement `update_graph(&self, provider_id: &str, window_type: &str)` — queries SQLite, renders with plotters
- [ ] 7.6 Connect dropdown change signals to `update_graph()`
- [ ] 7.7 Add "Refresh" button to manually reload data
- [ ] 7.8 Handle window close (hide, don't destroy — reuse on next open)

## 8. Plotters Graph Rendering

- [ ] 8.1 Add `plotters` dependency to `Cargo.toml` with `bitmap_backend`, `line_series`, `area_series`
- [ ] 8.2 Create `src/ui/graph_renderer.rs`
- [ ] 8.3 Implement `render_daily_trend(data: &[UsagePoint], provider_name: &str) -> Result<Vec<u8>>` — returns PNG bytes
  - X-axis: last 7 days (Mon, Tue, Wed...)
  - Y-axis: 0-100% utilization
  - Line series with points
  - Title: "{provider_name} - Daily Trend"
- [ ] 8.4 Implement `render_hourly_progress(data: &[UsagePoint], window_hours: u32) -> Result<Vec<u8>>`
  - X-axis: hours in window (00, 06, 12, 18...)
  - Y-axis: 0-100% utilization
  - Area series (filled under line)
  - Title: "Current Window Progress"
- [ ] 8.5 Handle empty data: render placeholder "No data available"
- [ ] 8.6 Add CSS styling: font sizes, colors, grid lines
- [ ] 8.7 Test with various data sizes (1 day, 7 days, partial data)

## 9. Integration: Polling Loop + Storage

- [ ] 9.1 Update polling task in `src/main.rs` or `src/tray.rs`:
  ```rust
  async fn polling_loop(config: Config, tx: Sender<Vec<ProviderData>>) {
      let history_store = HistoryStore::new(&config.history.db_path)?;
      loop {
          interval.tick().await;
          let data = fetch_all(&config).await;
          if config.history.enabled {
              for entry in &data {
                  history_store.insert_snapshot(entry)?;
              }
          }
          tx.send(data).await?;
      }
  }
  ```
- [ ] 9.2 Store snapshots asynchronously (don't block polling)
- [ ] 9.3 Add cleanup task: delete snapshots older than `retention_days`
- [ ] 9.4 Handle storage errors gracefully (log, continue)

## 10. Integration: UI Updates

- [ ] 10.1 Update popover to show provider metadata (plan badge for Ollama, credits for Codex)
- [ ] 10.2 Add "View Usage History" button at bottom of popover
- [ ] 10.3 Wire button to open `HistoryWindow` (singleton — only one instance)
- [ ] 10.4 Pass `HistoryStore` reference to `HistoryWindow` for querying
- [ ] 10.5 Update system tray tooltip to show aggregate usage summary
- [ ] 10.6 Add "History enabled" indicator in settings (if history is disabled, button shows "History not enabled")

## 11. Build & Verify

- [ ] 11.1 Run `cargo check` — confirm no compile errors
- [ ] 11.2 Run `cargo test` — confirm unit tests pass (parser tests, DB tests)
- [ ] 11.3 Run `cargo build --release`
- [ ] 11.4 Install extension, restart GNOME Shell
- [ ] 11.5 Configure providers in `~/.config/ai-usage-indicator/config.toml`:
  ```toml
  [providers.claude_code]
  enabled = true

  [providers.ollama]
  enabled = true
  cookie = "session_id=..."

  [providers.codex]
  enabled = true

  [history]
  enabled = true
  retention_days = 30
  ```
- [ ] 11.6 Verify Claude Code data appears in popover
- [ ] 11.7 Verify Ollama Cloud data appears (with valid cookie)
- [ ] 11.8 Verify OpenAI Codex data appears (with valid OAuth token)
- [ ] 11.9 Open history window, verify daily trend graph renders
- [ ] 11.10 Verify hourly progress graph renders
- [ ] 11.11 Wait 24 hours, verify daily trend shows multiple days
- [ ] 11.12 Test error cases: expired cookie, expired token, no network

## 12. Documentation

- [ ] 12.1 Update `README.md` with provider setup instructions
- [ ] 12.2 Add "Getting Ollama Cloud Cookie" guide (step-by-step with screenshots)
- [ ] 12.3 Add "Getting OpenAI Codex OAuth Token" guide
- [ ] 12.4 Document config file format with examples
- [ ] 12.5 Add troubleshooting section (common errors, solutions)
