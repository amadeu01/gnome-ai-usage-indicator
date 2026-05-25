## 1. Rust: ProviderData Default and shared helpers

- [x] 1.1 Add `Default` impl for `ProviderData` in `src/providers/mod.rs` (empty strings for id/name, 0.0 utilization, None for all Options)
- [x] 1.2 Add `pub fn error_entry(id: &str, name: &str, msg: &str) -> ProviderData` in `src/providers/mod.rs`
- [x] 1.3 Rename `dirs_home()` to `pub fn home_dir()` in `src/config.rs`, make public

## 2. Rust: Migrate providers to use shared helpers

- [x] 2.1 Refactor `src/providers/claude_code.rs` — use `Default`, `super::error_entry()`, `crate::config::home_dir()`; delete local `error_entry()` and `cache_path()` HOME lookup
- [x] 2.2 Refactor `src/providers/anthropic_api.rs` — use `Default` and `super::error_entry()` for all construction sites
- [x] 2.3 Refactor `src/providers/ollama.rs` — use `Default` and `super::error_entry()` for all construction sites
- [x] 2.4 Refactor `src/providers/codex.rs` — use `Default`, `super::error_entry()`, `crate::config::home_dir()`; delete local `error_entry()` and `codex_dir()` HOME lookup

## 3. Rust: Verify

- [x] 3.1 Run `cargo check` — zero errors
- [x] 3.2 Run `cargo test` — all existing tests pass
- [x] 3.3 Verify JSON output unchanged by running daemon and calling `GetProviderData` over DBus

## 4. TypeScript: Dead code removal and simplifications

- [x] 4.1 Delete `ProviderId` type alias from `ts-src/src/types.ts`
- [x] 4.2 Delete `Soup` module declaration from `ts-src/src/ambient.d.ts`
- [x] 4.3 Delete `Clutter` module declaration from `ts-src/src/ambient.d.ts`
- [x] 4.4 Inline `_restartPolling()` in `ts-src/src/extension.ts` — replace call site with `this._startPolling()`
- [x] 4.5 Store `tilesBox` as `this._tilesBox` field in `UsagePopover`, delete `_findTilesBox()` method in `ts-src/src/usagePopover.ts`

## 5. TypeScript: Verify

- [x] 5.1 Run TypeScript type-check (`make typecheck`) — zero errors
