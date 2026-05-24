## Why

The current architecture has both the Rust app and the GNOME Extension independently fetching AI provider data, duplicating API calls and config logic. On GNOME Wayland, the Rust GTK4 window cannot anchor to the top-right corner (Wayland blocks absolute positioning, GNOME rejects layer-shell), so the centered floating window is an unsatisfying UX that forces `GDK_BACKEND=x11` as a workaround.

## What Changes

- Rust app becomes a headless background daemon — no GTK window, no tray icon, pure async tokio process
- Rust daemon exposes a DBus interface (`io.github.amadeu01.AiUsageIndicator`) with provider data and config control
- GNOME Extension stops fetching AI APIs directly; switches `ProviderManager` to query Rust daemon via DBus
- Rust retains all provider implementations (Anthropic API, Claude Code, Codex) as the single source of truth
- `GDK_BACKEND=x11` workaround removed — no GTK in daemon
- `ksni` tray integration removed from Rust (extension renders everything in GNOME top bar)
- **BREAKING**: Rust binary changes role from GUI app to daemon; existing `ai-usage-indicator.service` unit file updated

## Capabilities

### New Capabilities

- `dbus-daemon`: Rust daemon process exposing `io.github.amadeu01.AiUsageIndicator` DBus service with provider data, refresh control, and config read/write
- `extension-dbus-client`: GNOME Extension DBus client replacing direct HTTP fetches in `ProviderManager`

### Modified Capabilities

- None — no existing `openspec/specs/` entries exist yet

## Impact

- **Rust**: Remove `gtk4`, `gdk4-x11`, `x11`, `libadwaita`, `ksni` deps; add `zbus` for DBus; remove `src/window.rs`, `src/tray.rs`, `src/ui/`; rewrite `src/main.rs` as daemon entry point
- **GNOME Extension**: Replace `ts-src/src/providerManager.ts` fetch logic with DBus proxy calls; remove per-provider HTTP code from extension
- **systemd**: Update `ai-usage-indicator.service` — daemon runs without display server dependency
- **Config**: Rust daemon owns config file; extension reads/writes config via DBus methods
