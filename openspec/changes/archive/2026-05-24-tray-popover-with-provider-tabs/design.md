## Context

The app is a Rust GTK4 system-tray indicator using `ksni` for the tray icon and `gtk4::ApplicationWindow` for the popup. The window currently opens at a position GTK4 chooses (typically center-screen). The `ksni::Tray::activate` callback receives `(x, y)` screen coordinates of the click but they are discarded. The content area is a single `gtk4::Box` with provider groups stacked vertically.

Linux desktops using Wayland do not expose absolute window positioning to client apps in most compositors; however the app targets X11/XWayland where `window.move_(x, y)` works, or falls back gracefully. The tray icon size is typically 22–24 px.

## Goals / Non-Goals

**Goals:**
- Window opens anchored below the tray icon rather than at a random position
- Tab bar at top of window — one tab per unique provider `name` — so users can switch providers without scrolling
- Cost/spend fields (`tokens_used`, `cost_usd`) added to `ProviderData` and displayed in the UI when populated

**Non-Goals:**
- True Wayland layer-shell positioning (requires `gtk4-layer-shell`; deferred)
- Auto-discovery of tray icon geometry via StatusNotifierItem protocol (complex; we use the click coords)
- Fetching cost data from remote APIs — we only surface what the provider module provides

## Decisions

### D1: Use click coordinates from `ksni::activate` to position window

`ksni::Tray::activate(x, y)` already receives the screen position of the click. We send `(x, y)` alongside the toggle signal so the window can call `window.move_(x - width/2, y + icon_height)`.

**Alternative considered**: Query the StatusNotifierHost for icon geometry. More accurate but requires D-Bus calls and is fragile across DEs. Click coords are good enough and already available.

### D2: Use `gtk4::Notebook` for the tab bar

`gtk4::Notebook` provides a native tab strip. We group `ProviderData` entries by `name`, create one `NotebookPage` per group, and set the tab label to the provider name.

**Alternative considered**: Custom `gtk4::Box` with `ToggleButton` tabs. More styling control but more code and no accessibility benefits.

### D3: Keep `ProviderData` as flat struct, add optional cost fields

Add `tokens_used: Option<u64>` and `cost_usd: Option<f64>` to `ProviderData`. Providers that don't have cost data leave them `None`; the UI skips the cost row when both are `None`.

**Alternative considered**: Separate `CostData` sub-struct. Over-engineered for two fields.

### D4: Position window: center-x on icon, below icon

`window.move_(icon_x - window_width/2, icon_y + 24)` where `icon_y` is the click Y. Clamp to screen bounds using `gdk::Monitor` geometry to avoid off-screen windows.

## Risks / Trade-offs

- [Wayland absolute positioning not supported] → Fall back to letting GTK4 pick position; window still appears, just not anchored. Add a comment in code so future work can add `gtk4-layer-shell`.
- [Notebook tab ordering changes if providers reload] → Preserve tab order by sorting provider names alphabetically; order is stable across refreshes.
- [Click X/Y may refer to icon center or edge depending on DE] → Offset by `icon_size/2` downward; visual result acceptable even if slightly off.

## Migration Plan

No persistent state changes. No config format changes. Deploy by rebuilding the binary. Rollback by reverting to previous binary.
