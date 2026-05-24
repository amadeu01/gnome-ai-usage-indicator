## MODIFIED Requirements

### Requirement: Claude Code local file data source
The extension SHALL read Claude Code usage data from `~/.claude/statusline-usage-cache.json`. It SHALL parse `five_hour`, `seven_day`, and `extra_usage` fields and emit one `ProviderData` entry per present window plus one extra entry if `extra_usage.is_enabled`. Each window entry SHALL include `windowLabel` and (for `seven_day`) `paceInfo`.

#### Scenario: Usage files present
- **WHEN** `~/.claude/statusline-usage-cache.json` exists and contains valid JSON
- **THEN** provider emits session entry, weekly entry (when non-null), and extra entry (when enabled)

#### Scenario: Usage files absent
- **WHEN** `~/.claude/statusline-usage-cache.json` does not exist
- **THEN** Claude provider tile shows "No data — is Claude Code installed?"

#### Scenario: Malformed JSON in usage file
- **WHEN** the cache file contains invalid JSON
- **THEN** Claude Code entries show error state; no extension crash

## ADDED Requirements

### Requirement: ProviderData windowLabel and paceInfo fields
The `ProviderData` interface SHALL include two new optional fields: `windowLabel?: string` (human-readable window name, e.g. "Session", "Weekly") and `paceInfo?: { label: string; ahead: boolean } | null`.

#### Scenario: Fields absent on providers without windows
- **WHEN** Codex or Anthropic API provider data is fetched
- **THEN** both `windowLabel` and `paceInfo` are undefined or null

### Requirement: Ollama REST data source
The extension SHALL fetch `{ollamaHost}/api/ps` when the Ollama provider is enabled. It SHALL use `Soup.Session` with a 3-second timeout. The response SHALL be mapped to a `ProviderData` with `usedTokens: 0`, `limitTokens: 0`, and model names stored in a new `meta` string field for display.

#### Scenario: Ollama models returned
- **WHEN** `/api/ps` returns `{ models: [{ name: "llama3", size_vram: 4000000000 }] }`
- **THEN** entry has `meta: "llama3 (3.7 GB)"` and no error

#### Scenario: Request fails
- **WHEN** connection refused to Ollama host
- **THEN** entry has `error: "Not running"` and `meta: null`
