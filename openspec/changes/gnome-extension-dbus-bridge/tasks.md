## 1. Rust: Add zbus dependency and remove display deps

- [x] 1.1 Add `zbus = { version = "4", features = ["tokio"] }` to `Cargo.toml`
- [x] 1.2 Remove `gtk4`, `gdk4-x11`, `x11`, `libadwaita`, `ksni`, `async-channel` from `Cargo.toml`
- [x] 1.3 Run `cargo build` to verify deps resolve without GTK

## 2. Rust: Add Anthropic API and Codex providers

- [x] 2.1 Create `src/providers/anthropic_api.rs` — port HTTP fetch logic from `ts-src/src/providers/anthropicApi.ts`
- [x] 2.2 Create `src/providers/codex.rs` — port logic from `ts-src/src/providers/codex.ts`
- [x] 2.3 Register both providers in `src/providers/mod.rs` `fetch_all()` behind config flags

## 3. Rust: Implement DBus interface

- [x] 3.1 Create `src/dbus.rs` with `zbus::interface` struct `AiUsageIndicatorInterface`
- [x] 3.2 Implement `GetProviderData() → String` method (returns JSON-serialized `Vec<ProviderData>`)
- [x] 3.3 Implement `Refresh()` method (sends signal to fetch loop to wake immediately)
- [x] 3.4 Implement `GetConfig() → String` method (JSON config, API keys replaced with `present: bool`)
- [x] 3.5 Implement `DataUpdated` signal emission using `zbus::SignalContext` after each fetch
- [x] 3.6 Write unit test: serialize/deserialize `ProviderData` round-trip through JSON

## 4. Rust: Rewrite main.rs as headless daemon

- [x] 4.1 Remove GTK `Application` init, `PidFileGuard`, `GDK_BACKEND` env override from `main.rs`
- [x] 4.2 Add `zbus::ConnectionBuilder` session bus setup with well-known name `io.github.amadeu01.AiUsageIndicator`
- [x] 4.3 Implement tokio fetch loop: sleep `poll_interval_secs`, call `providers::fetch_all()`, cache result, emit `DataUpdated`
- [x] 4.4 Wire `Refresh()` via `tokio::sync::Notify` to interrupt fetch loop sleep
- [x] 4.5 Handle SIGHUP: reload config and restart fetch loop
- [x] 4.6 Run `cargo build --release` and verify binary starts without display

## 5. Rust: Delete GTK source files

- [x] 5.1 Delete `src/window.rs`
- [x] 5.2 Delete `src/tray.rs`
- [x] 5.3 Delete `src/ui/` directory
- [x] 5.4 Remove all `mod window`, `mod tray`, `mod ui` declarations from `src/main.rs`
- [x] 5.5 Run `cargo build` with no warnings or errors

## 6. Extension: Replace ProviderManager with DBus client

- [x] 6.1 Rewrite `ts-src/src/providerManager.ts`: replace HTTP fetch with `Gio.DBusProxy` connecting to `io.github.amadeu01.AiUsageIndicator`
- [x] 6.2 Implement `fetchAll()` via `GetProviderData()` DBus call + `JSON.parse`
- [x] 6.3 Subscribe to `DataUpdated` signal in `ProviderManager.constructor()` and emit `data-updated` on receipt
- [x] 6.4 Implement offline detection: if DBus call fails, set all providers to error state `"Daemon offline"`
- [x] 6.5 Implement reconnect: on next `fetchAll()` after failure, attempt to re-acquire proxy

## 7. Extension: Remove provider HTTP code

- [x] 7.1 Delete `ts-src/src/providers/anthropicApi.ts`
- [x] 7.2 Delete `ts-src/src/providers/claudeCode.ts`
- [x] 7.3 Delete `ts-src/src/providers/codex.ts`
- [x] 7.4 Remove `ts-src/src/providers/` directory (if empty)
- [x] 7.5 Run `pnpm run build` in `ts-src/` — zero TypeScript errors

## 8. Extension: Update GSettings schema

- [x] 8.1 Remove `anthropic-api-key` key from `ts-src/schemas/org.gnome.shell.extensions.ai-usage-indicator.gschema.xml`
- [x] 8.2 Verify `glib-compile-schemas ts-src/schemas/` succeeds

## 9. Extension: Update preferences page

- [x] 9.1 Remove API key `Adw.PasswordEntryRow` from `ts-src/src/prefs.ts`
- [x] 9.2 Add `Adw.ActionRow` with subtitle pointing to `~/.config/ai-usage-indicator/config.toml`
- [ ] 9.3 Build and open prefs — confirm no API key field shown

## 10. Systemd unit and install

- [x] 10.1 Update `ai-usage-indicator.service`: remove `DISPLAY`/`WAYLAND_DISPLAY` env, remove `After=graphical-session.target`, add `Restart=on-failure`
- [x] 10.2 Update `Makefile` install target: remove GTK resource install steps if any
- [ ] 10.3 Test: `systemctl --user start ai-usage-indicator` starts daemon, `busctl --user list` shows service name

## 11. End-to-end validation

- [ ] 11.1 Start daemon, enable extension in GNOME Extensions — confirm data appears in top bar
- [ ] 11.2 Stop daemon — confirm extension shows "Daemon offline" state in UI
- [ ] 11.3 Restart daemon — confirm extension resumes showing data within one poll cycle
- [ ] 11.4 Call `busctl --user call io.github.amadeu01.AiUsageIndicator /io/github/amadeu01/AiUsageIndicator io.github.amadeu01.AiUsageIndicator Refresh` — confirm `DataUpdated` signal fires

## 12. Documentation

- [ ] 12.1 Update `README.md`: remove GTK window section, add DBus daemon architecture diagram
- [ ] 12.2 Document config file format and API key setup in README
