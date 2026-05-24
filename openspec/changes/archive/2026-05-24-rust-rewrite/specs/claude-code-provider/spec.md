## ADDED Requirements

### Requirement: Parse statusline usage cache
The Claude Code provider SHALL read `~/.claude/statusline-usage-cache.json` and deserialise it using `serde_json`. It SHALL emit one `ProviderData` per non-null window (`five_hour`, `seven_day`) plus one extra entry if `extra_usage.is_enabled`.

#### Scenario: Both windows present
- **WHEN** `five_hour` and `seven_day` are non-null
- **THEN** provider emits two entries: `window_label: "Session"` (5h) and `window_label: "Weekly"` (7d)

#### Scenario: File absent
- **WHEN** `~/.claude/statusline-usage-cache.json` does not exist
- **THEN** provider returns a single error entry: `error: Some("No data — is Claude Code installed?")`

#### Scenario: File malformed
- **WHEN** the cache file contains invalid JSON
- **THEN** provider returns a single error entry with the parse error message; app does not panic

### Requirement: Pace calculation on weekly window
The weekly entry SHALL include a `pace_info` field computed from `utilization` and `resets_at`.

#### Scenario: resets_at present
- **WHEN** `seven_day.resets_at` is a valid RFC3339 timestamp
- **THEN** `pace_info` is computed: `pace_ratio = used_fraction / elapsed_fraction`; label is "Ahead (+X%)" if ratio > 1.0, "Behind (-X%)" if ratio < 0.8, "On track" otherwise

#### Scenario: resets_at absent
- **WHEN** `seven_day.resets_at` is null
- **THEN** `pace_info` is `None`; pace line not rendered

### Requirement: Extra usage credits entry
When `extra_usage.is_enabled` is `true`, provider SHALL emit an additional entry with `window_label: "Extra usage"`, `used_credits` and `limit_credits` populated.

#### Scenario: Extra usage enabled
- **WHEN** `extra_usage.is_enabled` is true and `monthly_limit` is 2000.0
- **THEN** entry has `limit_credits: Some(2000.0)`, `used_credits: Some(...)`, `window_label: Some("Extra usage")`

#### Scenario: Extra usage disabled
- **WHEN** `extra_usage.is_enabled` is false
- **THEN** no extra usage entry emitted
