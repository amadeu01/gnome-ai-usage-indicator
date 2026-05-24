## Why

The TypeScript/GJS GNOME Shell extension approach is constrained by the GJS sandbox: no real async runtime, no crates ecosystem, fragile resource paths that differ across GNOME versions, and a hard requirement to run inside the shell process. Rust with `ksni` + `gtk4-rs` gives a standalone binary with a real async runtime, Cargo dependency management, type-safe GObject bindings, and zero shell-version breakage risk.

## What Changes

- **BREAKING**: Remove entire TypeScript/GJS extension (`src/`, `metadata.json`, `schemas/`, `Makefile`, `tsconfig.json`, `package.json`)
- **BREAKING**: No longer a GNOME Shell extension — becomes a standalone tray application (requires AppIndicator support on GNOME, built-in on KDE/XFCE)
- Replace with a Rust binary using:
  - `ksni` for StatusNotifierItem tray icon (D-Bus protocol)
  - `gtk4-rs` + `libadwaita-rs` for the popover window on tray click
  - `tokio` async runtime for non-blocking provider fetches
  - `serde` / `serde_json` for parsing provider data files
  - `reqwest` for HTTP (Ollama, Anthropic API)
- Claude Code provider: reads `~/.claude/statusline-usage-cache.json`, parses session (5h) and weekly (7d) windows with pace indicator
- Ollama provider: polls `localhost:11434/api/ps` for running models
- Popover UI: per-provider sections with labelled usage windows, progress bars, reset countdowns — matching the CodexBar-style layout from the reference screenshot
- Preferences stored in `~/.config/ai-usage-indicator/config.toml` (replaces dconf/GSettings)
- Systemd user service unit for autostart (replaces GNOME extension autoload)

## Capabilities

### New Capabilities
- `tray-icon`: System tray icon via `ksni` (StatusNotifierItem); shows icon, handles left-click to toggle popover
- `usage-popover`: GTK4 + Adwaita popover window; per-provider sections with session/weekly usage bars, reset countdown, pace line
- `claude-code-provider`: Reads `~/.claude/statusline-usage-cache.json`; parses 5h/7d windows + extra usage credits
- `ollama-provider`: Polls Ollama REST API (`/api/ps`) for running models; shows model names + VRAM
- `config`: TOML config file at `~/.config/ai-usage-indicator/config.toml`; stores enabled providers, poll interval, Ollama host, Anthropic API key
- `autostart`: Systemd user service unit + `make install` target

### Modified Capabilities

(none — full rewrite, all prior specs superseded)

## Impact

- All TypeScript source deleted: `src/`, `dist/`, `node_modules/`, `tsconfig.json`, `package.json`, `Makefile`, `metadata.json`, `schemas/`
- New `Cargo.toml` + `src/` (Rust)
- New `Makefile` with `build`, `install`, `run`, `clean` targets using `cargo`
- Runtime dependency: `libgtk-4`, `libadwaita-1`, D-Bus (all standard on modern GNOME)
- Install target drops binary to `~/.local/bin/` and service to `~/.config/systemd/user/`
- GNOME users need AppIndicator/KStatusNotifierItem extension enabled (or GNOME 47+ which has it built-in via `gnome-shell-extension-appindicator`)
