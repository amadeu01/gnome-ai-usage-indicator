## 1. Data Model

- [x] 1.1 Add `tokens_used: Option<u64>` and `cost_usd: Option<f64>` fields to `ProviderData` in `src/providers/mod.rs`
- [x] 1.2 Update all `ProviderData` construction sites (claude_code.rs, ollama.rs) to include the new fields (set to `None` where not applicable)
- [x] 1.3 Parse optional `cost` block from Claude Code cache JSON in `src/providers/claude_code.rs` and populate `tokens_used` / `cost_usd`

## 2. Tray Anchor — Position Passing

- [x] 2.1 Change the toggle channel type from `async_channel::Sender<()>` to `async_channel::Sender<(i32, i32)>` in `src/main.rs` and `src/tray.rs`
- [x] 2.2 In `src/tray.rs` `activate`, send `(x, y)` instead of `()`
- [x] 2.3 Update the toggle receiver in `src/main.rs` to pass `(x, y)` to `win.show_at(x, y)`

## 3. Tray Anchor — Window Positioning

- [x] 3.1 Add `show_at(tray_x: i32, tray_y: i32)` method to `AppWindow` in `src/window.rs`
- [x] 3.2 Inside `show_at`: compute `win_x = tray_x - window_width/2`, `win_y = tray_y + 24`
- [x] 3.3 Clamp `win_x` and `win_y` to monitor work area using `gdk::Display::default()` monitor geometry
- [x] 3.4 Call `window.move_(win_x, win_y)` then `window.present()`
- [x] 3.5 Replace `toggle()` callers in `main.rs` with `show_at` / explicit hide logic

## 4. Provider Tab Bar

- [x] 4.1 Replace `content_box: Box` field in `AppWindow` with `notebook: gtk4::Notebook`
- [x] 4.2 In `AppWindow::new`, construct `gtk4::Notebook` with `show_border(false)` and add it to `root_vbox` in place of the `ScrolledWindow`
- [x] 4.3 Rewrite `update_data` in `src/window.rs`: remove all existing notebook pages, group entries by `name` (sorted alphabetically), create one `ScrolledWindow` + `Box` per group as notebook pages with tab labels
- [x] 4.4 Persist selected tab across refreshes: before clearing pages, record the current tab label; after rebuild, restore selection by matching label

## 5. UI — Cost Row

- [x] 5.1 In `src/ui/provider_group.rs` `build_entry_section`, add a cost row below the reset label when `tokens_used` or `cost_usd` is `Some`
- [x] 5.2 Format tokens: use "K" suffix for ≥1000, "M" suffix for ≥1_000_000 (e.g. "15K tokens", "218M tokens")
- [x] 5.3 Format cost as "$X.XX" (2 decimal places)
- [x] 5.4 Combine into single label: "218M tokens · $254.24" when both present, otherwise just the available field

## 6. Styling

- [x] 6.1 Add `.cost-label` CSS class rule to `src/style.css` (small, dimmed, similar to `.reset-label`)
- [x] 6.2 Style the notebook tab bar: remove default GTK borders, ensure tab labels use system font at 0.9em
- [ ] 6.3 Verify popover shadow and border-radius still render correctly after notebook replaces the inner box

## 7. Verification

- [ ] 7.1 Build and run: click tray icon, confirm window appears below icon
- [ ] 7.2 Verify tab switching works and content updates correctly
- [ ] 7.3 Verify cost row appears for Claude Code entries when cache has cost data
- [x] 7.4 Run `cargo test` — existing unit tests must pass
- [ ] 7.5 Test near-edge positioning: move tray icon to a secondary monitor edge and confirm clamping
