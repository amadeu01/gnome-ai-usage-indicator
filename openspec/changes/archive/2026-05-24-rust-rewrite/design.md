## Context

Current project is a GNOME Shell extension (TypeScript/GJS). It works but has three persistent pain points: GJS resource paths differ across GNOME versions (broke prefs on GNOME 50), no real async runtime (Promises work but microtask scheduling is opaque), and the extension lifecycle is tied to the shell process (a JS error can affect GNOME stability).

The rewrite targets a standalone Rust binary. The primary runtime is Tokio. The UI toolkit is GTK4 via `gtk4-rs`. The tray integration uses `ksni` (StatusNotifierItem D-Bus protocol), which works on any modern Linux DE without extension-specific hacks.

## Goals / Non-Goals

**Goals:**
- Single Rust binary, no Node.js/npm dependency
- Tray icon with clickable GTK4 popover (custom widgets, progress bars)
- Claude Code session + weekly usage with pace indicator
- Ollama running-models tile
- TOML config (no dconf)
- Systemd user service for autostart
- Cross-DE: works on GNOME (with AppIndicator), KDE, XFCE out of the box

**Non-Goals:**
- GNOME panel integration (left/center/right positioning) — requires GJS extension
- Wayland screenshot / screen capture
- Windows or macOS

## Decisions

### D1: ksni for tray, not libappindicator-rs
`ksni` implements the StatusNotifierItem D-Bus spec natively in Rust with no C library dependency. `libappindicator-sys` wraps a C library that is no longer maintained upstream. `ksni` is async-friendly (Tokio) and compiles to a single binary.

Rejected: `libappindicator-rs` — unmaintained, requires C headers.

### D2: GTK4 popover as a separate `gtk::Window` (not `gtk::Popover`)
`gtk::Popover` must be anchored to a widget in the same process. The tray icon is managed by the compositor (external process). Instead: create a `gtk::Window` with `decorated: false`, `type_hint: PopupMenu`, positioned near the cursor on activation. This is the standard approach for tray popup UIs in GTK4.

Rejected: `gtk::Popover` — can't anchor to an external tray icon.

### D3: Tokio for async, not GLib main loop
GTK4 requires the GLib main loop for UI. Tokio is used for background tasks (file reads, HTTP). Bridge: `glib::spawn_future_local` for GTK-thread tasks; `tokio::spawn` for background fetches that send results back via `glib::MainContext::channel` (a GLib-compatible mpsc). This avoids blocking the GTK thread on network I/O.

### D4: `reqwest` for HTTP (Ollama, Anthropic API)
`reqwest` with `tokio` backend is the standard choice. Alternative `ureq` is sync-only, which would block Tokio threads. `hyper` directly is too low-level. `reqwest` + `tokio` gives connection pooling, timeout support, and JSON deserialization via `serde_json`.

### D5: TOML config via `toml` crate, stored at `~/.config/ai-usage-indicator/config.toml`
Replaces GSettings/dconf. `toml` is a first-class Rust crate, human-editable, no schema compilation step. Preferences UI writes directly to the file; the main loop re-reads on SIGHUP or file watcher event.

### D6: Module structure
```
src/
  main.rs          — Tokio entry point, GTK init, tray service, poll loop
  tray.rs          — ksni Tray impl, left-click → show/hide window
  window.rs        — GTK4 Window, popover layout, update_data()
  config.rs        — Config struct, load/save, defaults
  providers/
    mod.rs         — ProviderData struct, fetch_all()
    claude_code.rs — reads ~/.claude/statusline-usage-cache.json
    ollama.rs      — GET /api/ps
  ui/
    provider_group.rs — builds per-provider widget group
    usage_bar.rs      — progress bar widget
```

### D7: Data flow
```
Tokio interval (poll_interval)
  → fetch_all() (async, all providers in parallel via tokio::join!)
  → sends Vec<ProviderData> through glib::MainContext::channel
  → GTK thread receives → window.update_data(data)
  → rebuilds provider group widgets
```

## Risks / Trade-offs

- [GNOME AppIndicator required] On stock GNOME 46 and below, the tray area is hidden unless `gnome-shell-extension-appindicator` is enabled. → Document in README; GNOME 47+ has it built-in.

- [GTK4 window positioning] `gtk::Window` with `PopupMenu` hint is positioned by the compositor; exact pixel alignment to the tray icon is compositor-dependent. → Accept slight misalignment; most compositors handle it correctly.

- [ksni + Tokio thread model] `ksni::TrayService::spawn()` runs on a Tokio task; GTK must run on the main thread. `glib::MainContext::channel` is the correct bridge but adds boilerplate. → Establish the pattern once in `main.rs`, reuse throughout.

- [Config file vs GSettings] No live dconf watch means prefs changes require app restart (or SIGHUP). → Add `notify::RecommendedWatcher` (from `notify` crate) on the config file to hot-reload; mark as stretch goal.

## Migration Plan

1. `git mv src/ ts-src/` to archive TypeScript (keep for reference during rewrite)
2. `cargo init` at repo root, add dependencies
3. Implement + verify providers independently (unit tests, no UI)
4. Wire GTK4 window, test popover layout
5. Wire ksni tray + left-click toggle
6. Delete `ts-src/`, `node_modules/`, `package.json`, `tsconfig.json` when Rust binary is feature-complete
7. `make install` installs binary + systemd unit; user disables old GNOME extension

Rollback: the GNOME extension remains in `~/.local/share/gnome-shell/extensions/` until explicitly removed; user can re-enable it independently.

## Open Questions

- Should the popover support keyboard navigation (Tab through providers)? GTK4 has good a11y support, but implementing full keyboard nav is non-trivial. Defer to post-MVP.
- GNOME 47+ has StatusNotifierItem support built-in — worth testing on the user's current GNOME 50.1 to confirm no AppIndicator extension is needed.
