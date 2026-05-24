## 1. Repo Scaffold

- [x] 1.1 Archive TypeScript: `git mv src ts-src && git mv metadata.json ts-src/ && git mv schemas ts-src/ && git mv stylesheet.css ts-src/`
- [x] 1.2 Run `cargo init --name ai-usage-indicator` at repo root
- [x] 1.3 Add dependencies to `Cargo.toml`: `tokio` (full), `gtk4`, `libadwaita`, `ksni`, `serde`, `serde_json`, `reqwest` (json feature), `toml`, `chrono`, `anyhow`
- [x] 1.4 Create module files: `src/tray.rs`, `src/window.rs`, `src/config.rs`, `src/providers/mod.rs`, `src/providers/claude_code.rs`, `src/providers/ollama.rs`, `src/ui/mod.rs`, `src/ui/provider_group.rs`, `src/ui/usage_bar.rs`
- [x] 1.5 Write new `Makefile` with `build`, `install`, `run`, `uninstall`, `clean` targets using `cargo build --release`
- [x] 1.6 Create `ai-usage-indicator.service` systemd user unit file pointing to `~/.local/bin/ai-usage-indicator`

## 2. Config

- [x] 2.1 Define `Config` struct in `src/config.rs` with all fields from spec (`poll_interval_secs`, `enabled_providers`, `ollama_host`, `anthropic_api_key`)
- [x] 2.2 Implement `Config::load()`: read from `~/.config/ai-usage-indicator/config.toml`; create with defaults if absent
- [x] 2.3 Implement `Config::save()`: serialise to TOML and write file (create parent dirs if needed)
- [x] 2.4 Wire SIGHUP handler in `main.rs` using `tokio::signal::unix` to reload config atomically

## 3. Provider Data Types

- [x] 3.1 Define `ProviderData` struct in `src/providers/mod.rs`: `id`, `name`, `window_label`, `utilization` (f32 0–100), `reset_at` (Option<DateTime<Utc>>), `pace_info` (Option<PaceInfo>), `used_credits`, `limit_credits`, `meta`, `error`
- [x] 3.2 Define `PaceInfo` struct: `label: String`, `ahead: bool`
- [x] 3.3 Define `fetch_all(config: &Config) -> Vec<ProviderData>` in `src/providers/mod.rs` using `tokio::join!` for parallel fetches

## 4. Claude Code Provider

- [x] 4.1 Define serde structs for `statusline-usage-cache.json` schema in `src/providers/claude_code.rs`
- [x] 4.2 Implement `fetch_claude_code() -> Vec<ProviderData>`: read and parse cache file; return error entry if file absent or malformed
- [x] 4.3 Map `five_hour` → entry with `window_label: "Session"`, utilization from field
- [x] 4.4 Map `seven_day` → entry with `window_label: "Weekly"`, compute `pace_info` when `resets_at` is present (window duration = 7 × 86400s)
- [x] 4.5 Map `extra_usage` → entry with `window_label: "Extra usage"` when `is_enabled`; populate `used_credits`/`limit_credits`

## 5. Ollama Provider

- [x] 5.1 Define serde structs for `/api/ps` response in `src/providers/ollama.rs`
- [x] 5.2 Implement `fetch_ollama(host: &str) -> ProviderData` using `reqwest` with 3-second timeout
- [x] 5.3 Map non-empty `models` array to `meta` string: `"name (X.X GB)"` per model, comma-joined
- [x] 5.4 Return `error: Some("Not running")` on connection failure or timeout
- [x] 5.5 Return `meta: Some("No models loaded")` on empty models array

## 6. GTK4 Popover Window

- [x] 6.1 Implement `build_usage_bar(utilization: f32) -> gtk::ProgressBar` in `src/ui/usage_bar.rs`; apply CSS classes `usage-ok`, `usage-warning`, `usage-critical` at 0/80/95 thresholds
- [x] 6.2 Implement `build_provider_group(entries: &[ProviderData]) -> gtk::Box` in `src/ui/provider_group.rs`; render provider name header, then one subsection per entry
- [x] 6.3 Each subsection: `window_label` as bold label, progress bar (skip for Ollama/no-limit entries), usage text, reset countdown, pace line (when `pace_info` present), model names (Ollama `meta`)
- [x] 6.4 Implement `AppWindow` in `src/window.rs`: `gtk::ApplicationWindow`, undecorated, `PopupMenu` window type hint, fixed width ~340px
- [x] 6.5 Add popover header: "AI Usage" title + refresh `gtk::Button`
- [x] 6.6 Add scrollable `gtk::ScrolledWindow` for provider groups; max height 500px
- [x] 6.7 Add footer `gtk::Label` with last-updated text; update every 30s via `glib::timeout_add_seconds_local`
- [x] 6.8 Implement `AppWindow::update_data(&self, data: Vec<ProviderData>)`: rebuild provider group widgets
- [x] 6.9 Implement `AppWindow::refresh_click`: disable button, spawn `fetch_all` on Tokio, send result back via `glib::MainContext::channel`, re-enable button on completion
- [x] 6.10 Write `src/style.css` with provider group, bar, pace, and footer styles; load via `gtk::CssProvider`
- [x] 6.11 Close window on focus-out: connect `notify::is-active` signal; hide when focus lost

## 7. Tray Icon

- [x] 7.1 Implement `AppTray` struct in `src/tray.rs` implementing `ksni::Tray`: `id()`, `title()`, `icon_name()`, `activate()` (toggle window)
- [x] 7.2 Wire `activate()` to show/hide the `AppWindow` via a shared `Arc<Mutex<bool>>` visible flag
- [x] 7.3 Implement tooltip showing max utilization across all providers
- [x] 7.4 Update tray icon name (`-warning`, `-critical` variants) when max utilization crosses thresholds

## 8. Main Entry Point

- [x] 8.1 In `main.rs`: init GTK (`gtk::init()`), create `gtk::Application`, set up `glib::MainContext::channel` for data updates
- [x] 8.2 Spawn `ksni::TrayService` on Tokio runtime
- [x] 8.3 Spawn Tokio interval loop: every `config.poll_interval_secs`, call `fetch_all()`, send result through channel
- [x] 8.4 GTK main loop receives `ProviderData` via channel receiver; calls `window.update_data()` and `tray.update_tooltip()`
- [x] 8.5 Wire SIGHUP reload: re-read config, update interval, update enabled providers

## 9. Build & Verify

- [x] 9.1 Run `cargo build` — confirm zero errors and warnings
- [x] 9.2 Run `cargo test` — all provider unit tests pass
- [x] 9.3 Run `make install` — binary lands in `~/.local/bin/`; systemd service installed
- [x] 9.4 Start app — tray icon appears in GNOME panel
- [ ] 9.5 Left-click tray — popover opens with Claude Code Session + Weekly tiles
- [ ] 9.6 Start `ollama serve` + load a model — Ollama tile shows model name and VRAM
- [ ] 9.7 Stop Ollama — tile updates to "Not running" on next poll
- [ ] 9.8 Edit config file; send SIGHUP — poll interval updates without restart
- [ ] 9.9 Delete `ts-src/` after confirming Rust binary is feature-complete
