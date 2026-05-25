## Context

The Rust daemon has 4 provider modules (`claude_code`, `anthropic_api`, `ollama`, `codex`) that each construct `ProviderData` structs. The struct has 11 fields, most defaulting to `None`/`0.0`. Every construction site spells out all 11 fields — producing ~12 near-identical blocks. Two modules duplicate an `error_entry()` helper. Three modules independently resolve `$HOME`. The TypeScript GNOME extension carries unused type declarations and a trivial method wrapper.

## Goals / Non-Goals

**Goals:**
- Eliminate repeated `ProviderData` construction boilerplate via `Default` impl + struct update syntax
- Consolidate shared helpers (`error_entry`, `home_dir`) into one location
- Remove dead TypeScript code (`ProviderId`, `Soup`/`Clutter` decls)
- Remove trivial wrappers that add no behavior

**Non-Goals:**
- Changing the DBus contract or serialized JSON shape
- Refactoring `fetch_all()` dispatch pattern (deferred — works fine, low ROI)
- Renaming TS `usedTokens`/`limitTokens` to match Rust semantics (separate change)
- Adding new providers or features
- Changing `Cargo.lock` gitignore policy

## Decisions

### 1. `Default` for `ProviderData` over a builder pattern

Add `#[derive(Default)]` won't work because `id` and `name` need meaningful defaults. Manual `Default` impl with empty strings for `id`/`name`, `0.0` for `utilization`, `None` for everything else.

**Why not builder?** Builders add a new abstraction. Struct update syntax (`ProviderData { id: "x".into(), ..Default::default() }`) is idiomatic Rust, zero overhead, no new types.

### 2. Shared `error_entry(id, name, msg)` in `providers/mod.rs`

Move the helper to the parent module with `id` and `name` as parameters. Both `claude_code.rs` and `codex.rs` call it. `anthropic_api.rs` and `ollama.rs` inline their error structs — those also switch to this helper.

**Why `mod.rs` not a new file?** It's 3 lines. A new `util.rs` for one function is over-organization.

### 3. Shared `home_dir()` in `config.rs`, re-exported

`config.rs` already has `dirs_home()`. Rename to `pub fn home_dir()`, use from all provider modules. Single source of truth for `$HOME` fallback behavior.

**Why not a `util` module?** `config` is the natural owner of environment/path resolution. Adding a `util` module for one function violates KISS.

### 4. Delete dead TS code directly

`ProviderId` type has zero imports. `Soup` and `Clutter` ambient modules have zero imports. Safe to remove — no runtime or compile-time impact.

### 5. Store `tilesBox` as field instead of walking children

`UsagePopover._findTilesBox()` iterates children by name string every refresh. Store the reference at construction time as `this._tilesBox`. Delete `_findTilesBox()`.

## Risks / Trade-offs

- **[Compile breakage from `Default` migration]** → Mitigated by doing one file at a time and running `cargo check` after each.
- **[Removing ambient.d.ts types that might be needed later]** → They're trivially re-addable from GJS docs. Dead code now shouldn't be kept for hypothetical future use.
- **[Changing `home_dir()` fallback behavior]** → All three current implementations use identical logic (`$HOME` or `/tmp`). No behavioral change.
