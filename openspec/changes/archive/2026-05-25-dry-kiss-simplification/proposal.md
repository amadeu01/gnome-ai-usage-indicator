## Why

The codebase has significant boilerplate and duplication stemming from `ProviderData` construction. Every provider file repeats 11 struct fields (mostly `None`/`0.0`) at every construction site — totaling ~12 near-identical blocks across 4 files. Helper functions are duplicated, HOME directory lookups are scattered, and the TypeScript side carries dead type declarations. This cleanup reduces maintenance surface and makes adding new providers trivial.

## What Changes

- Add `Default` impl for `ProviderData` and use struct update syntax (`..Default::default()`) at all construction sites, eliminating ~120 lines of boilerplate
- Extract shared `error_entry(id, name, msg)` into `providers/mod.rs`, delete copies from `claude_code.rs` and `codex.rs`
- Extract shared `home_dir() -> PathBuf` helper, replacing 3 independent `HOME` env lookups in `config.rs`, `claude_code.rs`, `codex.rs`
- Delete dead TypeScript code: `ProviderId` type, unused `Soup`/`Clutter` ambient declarations
- Inline trivial `_restartPolling()` wrapper in `extension.ts`
- Store `tilesBox` reference in `UsagePopover` instead of walking children by name each refresh

## Capabilities

### New Capabilities
- `provider-data-defaults`: Default implementation for ProviderData and shared construction helpers (error_entry, home_dir)
- `dead-code-removal`: Removal of unused TypeScript types and ambient declarations

### Modified Capabilities

## Impact

- **Rust providers**: All 4 provider files (`claude_code.rs`, `anthropic_api.rs`, `ollama.rs`, `codex.rs`) and `providers/mod.rs` modified
- **Rust config**: `config.rs` gains shared `home_dir()` or delegates to it
- **TypeScript**: `types.ts`, `ambient.d.ts`, `extension.ts`, `usagePopover.ts` modified
- **No API/DBus contract changes**: All serialized shapes remain identical
- **No dependency changes**
