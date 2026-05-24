## Context

Current state: two independent subsystems that both fetch AI provider data.

1. **Rust GTK4 app** (`src/`) — fetches Claude Code and Ollama data, renders a GTK4 window, registers a `ksni` tray icon. Forces `GDK_BACKEND=x11` to work around Wayland's no-absolute-positioning constraint. Providers: `claude_code`, `ollama`.

2. **GNOME Extension** (`ts-src/`) — fetches Anthropic API, Claude Code, and Codex directly from GJS via HTTP. Renders in the GNOME top bar. Providers: `anthropicApi`, `claudeCode`, `codex`.

The two halves are uncoordinated: separate polling loops, separate config, separate API keys. The Rust GTK window only works acceptably under X11/XWayland; under native Wayland it opens centered with no way to anchor to the panel.

## Goals / Non-Goals

**Goals:**
- Single source of truth for data: Rust daemon fetches all providers, caches results, exposes via DBus
- GNOME Extension becomes a pure display layer — no HTTP, no API keys in GSettings
- Proper top-bar integration without X11 hacks
- Daemon survives GNOME Shell restart (extension re-connects via DBus on re-enable)
- Systemd unit keeps daemon running independently of display session

**Non-Goals:**
- Supporting non-GNOME compositors (Sway, KDE) — out of scope
- Replacing the GNOME Extension with a pure GTK4 window (that's Option A, not chosen)
- Multi-user or sandboxed (Flatpak) deployment in this iteration

## Decisions

### 1. DBus library: `zbus` (async, pure Rust)

`zbus` over `dbus-rs` because: pure Rust (no libdbus C dep), first-class async/tokio integration, derive macros for interface definition. Session bus (not system bus) — data is per-user, no root required.

**Interface name**: `io.github.amadeu01.AiUsageIndicator`
**Object path**: `/io/github/amadeu01/AiUsageIndicator`

### 2. DBus API shape

```
Interface: io.github.amadeu01.AiUsageIndicator

Methods:
  GetProviderData() → (s json_array)   // JSON-serialized Vec<ProviderData>
  Refresh()                             // trigger immediate re-fetch
  GetConfig() → (s json_object)        // serialized daemon config
  SetConfig(s json_object)             // write config, reload providers

Signals:
  DataUpdated(s json_array)            // emitted after each fetch cycle
```

JSON over typed DBus variants chosen because `ProviderData` has optional fields and evolves frequently — avoids DBus signature churn. Extension deserializes with `JSON.parse`.

### 3. Daemon process model

- Pure `tokio` runtime, no GTK, no display dependency
- Background fetch loop at configurable interval (default 60s)
- Holds `zbus::Connection` on session bus; serves requests inline in async tasks
- On `Refresh()` call: cancels current sleep, re-fetches immediately
- Config stored at `~/.config/ai-usage-indicator/config.toml` (same path as before)

### 4. Extension DBus client

Replace `ProviderManager.fetchAll()` HTTP logic with a `Gio.DBusProxy` call to `GetProviderData()`. Subscribe to `DataUpdated` signal for push updates. Keep the same `ProviderData` TypeScript interface — only the transport changes.

API key (Anthropic) moves from GSettings into the daemon config file. Extension no longer needs the `anthropic-api-key` GSettings key.

### 5. Provider consolidation

Rust daemon absorbs the Anthropic API and Codex providers currently in the extension. The extension's `ts-src/src/providers/` directory is deleted. Single provider registry in Rust.

### 6. Removing GTK from Rust

Delete: `src/window.rs`, `src/tray.rs`, `src/ui/`. Remove `Cargo.toml` deps: `gtk4`, `gdk4-x11`, `x11`, `libadwaita`, `ksni`, `async-channel`. Add: `zbus`.

### 7. Systemd unit

Remove `DISPLAY` / Wayland env requirements from service. Unit type changes from `simple` (GTK app) to `simple` daemon — no `After=graphical-session.target` needed. Add `Restart=on-failure`.

## Risks / Trade-offs

- **Extension loses data when daemon is stopped** → Mitigation: extension shows "daemon offline" state with last cached value; retry connection on next poll tick
- **DBus session bus not available in some minimal sessions** → Mitigation: document requirement; daemon logs clear error and exits
- **JSON serialization overhead** → Negligible at 60s poll interval with <10 providers
- **Anthropic API key now in config file, not GSettings** → Migration needed; extension prefs page updated to link to config file docs
- **zbus adds compile-time dependency** → Acceptable; pure Rust, well-maintained

## Migration Plan

1. Add `zbus` to `Cargo.toml`; implement daemon DBus interface
2. Update `src/main.rs`: remove GTK init, start tokio runtime + DBus server + fetch loop
3. Delete `src/window.rs`, `src/tray.rs`, `src/ui/`
4. Update extension `ProviderManager` to use `Gio.DBusProxy`
5. Remove provider HTTP code from extension (`ts-src/src/providers/`)
6. Update GSettings schema: remove `anthropic-api-key`, add `daemon-bus-name` (or hardcode)
7. Update `ai-usage-indicator.service` systemd unit
8. Update `Makefile` install targets
9. Update README

Rollback: revert to previous git tag — the X11 workaround path still works.

## Open Questions

- Should Codex provider be ported to Rust (needs HTTP token auth details) or dropped for now?
- Should the daemon expose a `SetConfig` method, or is editing the TOML file sufficient for v1?
