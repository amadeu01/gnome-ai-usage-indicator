## ADDED Requirements

### Requirement: ProviderData has a Default implementation
The `ProviderData` struct SHALL implement `Default` with `id` and `name` as empty strings, `utilization` as `0.0`, and all `Option` fields as `None`.

#### Scenario: Constructing a provider entry with minimal fields
- **WHEN** a provider module creates a `ProviderData` with only `id`, `name`, and `utilization` set
- **THEN** all other fields SHALL default to `None` via struct update syntax (`..Default::default()`)

#### Scenario: Existing serialized output unchanged
- **WHEN** any provider constructs `ProviderData` using defaults
- **THEN** the JSON output over DBus SHALL be byte-identical to the previous explicit construction

### Requirement: Shared error_entry helper in providers module
The `providers/mod.rs` module SHALL expose a `pub fn error_entry(id: &str, name: &str, msg: &str) -> ProviderData` function that constructs an error-state `ProviderData` with the given message and defaults for all other fields.

#### Scenario: Claude Code uses shared error_entry
- **WHEN** `claude_code.rs` needs to return an error state
- **THEN** it SHALL call `super::error_entry("claude-code", "Claude Code", msg)` instead of a local `error_entry()` function

#### Scenario: Codex uses shared error_entry
- **WHEN** `codex.rs` needs to return an error state
- **THEN** it SHALL call `super::error_entry("codex", "Codex", msg)` instead of a local `error_entry()` function

#### Scenario: Other providers use shared error_entry for error states
- **WHEN** `anthropic_api.rs` or `ollama.rs` constructs an error-state `ProviderData`
- **THEN** it SHALL use `super::error_entry()` instead of inline struct literals

### Requirement: Shared home_dir helper
A single `pub fn home_dir() -> PathBuf` function SHALL exist in `config.rs` that returns `$HOME` or falls back to `/tmp`. All modules that resolve the user's home directory SHALL use this function.

#### Scenario: claude_code uses shared home_dir
- **WHEN** `claude_code.rs` resolves the cache file path
- **THEN** it SHALL call `crate::config::home_dir()` instead of reading `$HOME` directly

#### Scenario: codex uses shared home_dir
- **WHEN** `codex.rs` resolves the codex directory
- **THEN** it SHALL call `crate::config::home_dir()` instead of reading `$HOME` directly

#### Scenario: config uses its own home_dir
- **WHEN** `config.rs` resolves the config file path
- **THEN** it SHALL use its own `home_dir()` function (replacing the previous `dirs_home()`)
