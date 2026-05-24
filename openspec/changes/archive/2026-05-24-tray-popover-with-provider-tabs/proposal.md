## Why

The app window opens at a fixed screen position unrelated to the tray icon, making it feel disconnected and hard to find. There is also no way to switch between AI providers quickly — all data is listed in a single scrollable column with no hierarchy.

## What Changes

- Window now positions itself directly below the tray icon on activation (popover behaviour matching CodexBar/macOS menu extras)
- Replace the flat scrollable list with a tab bar: one tab per provider (Claude Code, Ollama, future providers)
- Each tab shows: usage bars per window (Session / Weekly), reset countdown timers, pace info, cost/spend section (tokens used, money spent this month)
- Add `tokens_used` and `cost_usd` fields to `ProviderData` so each provider can surface spend data
- Tray icon click passes its geometry to the window so it can anchor beneath the icon

## Capabilities

### New Capabilities
- `tray-anchored-window`: Window positions below the tray icon geometry on every open; stays on screen if near an edge
- `provider-tab-bar`: Tab strip at top of window, one tab per provider name; selecting a tab shows only that provider's data
- `usage-cost-display`: Per-provider section showing tokens consumed and USD cost for the current billing period alongside existing utilization bars

### Modified Capabilities
- (none — no existing specs to delta)

## Impact

- `src/tray.rs`: must capture and forward `(x, y)` from `activate` to the window
- `src/window.rs`: new `position_below_tray(x, y, icon_size)` method; swap `Box` content area for `gtk4::Notebook` (tab widget)
- `src/providers/mod.rs`: add `tokens_used: Option<u64>` and `cost_usd: Option<f64>` to `ProviderData`
- `src/providers/claude_code.rs`: populate cost fields from cache JSON if present
- `src/ui/provider_group.rs`: extend entry section to show cost row when data available
- `src/style.css`: tab bar styling, cost row styling
- No new crate dependencies required (GTK4 Notebook is already in scope)
