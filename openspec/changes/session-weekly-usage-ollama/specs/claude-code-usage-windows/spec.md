## ADDED Requirements

### Requirement: Session and weekly windows parsed as distinct entries
The Claude Code provider SHALL emit two separate `ProviderData` entries from `statusline-usage-cache.json`: one for `five_hour` (Session) and one for `seven_day` (Weekly). Each entry SHALL carry a `windowLabel` field (`"Session"` and `"Weekly"` respectively).

#### Scenario: Both windows present
- **WHEN** `five_hour` and `seven_day` are both non-null in the cache file
- **THEN** provider emits two entries: id `claude-code-session` with `windowLabel: "Session"` and id `claude-code-weekly` with `windowLabel: "Weekly"`

#### Scenario: One window absent
- **WHEN** `seven_day` is null but `five_hour` is present
- **THEN** only the session entry is emitted; no weekly entry; no error shown

### Requirement: Pace indicator on weekly window
The weekly entry SHALL include a `paceInfo` field describing whether usage is ahead or behind a linear pace through the 7-day window.

#### Scenario: Ahead of pace
- **WHEN** elapsed fraction of 7d window is 0.30 and utilization is 0.50 (pace ratio 1.67)
- **THEN** `paceInfo.label` is `"Ahead (+67%)"` and `paceInfo.ahead` is `true`

#### Scenario: Behind pace
- **WHEN** elapsed fraction of 7d window is 0.50 and utilization is 0.20 (pace ratio 0.40)
- **THEN** `paceInfo.label` is `"Behind (-60%)"` and `paceInfo.ahead` is `false`

#### Scenario: Reset time unknown, pace omitted
- **WHEN** `seven_day.resets_at` is null
- **THEN** `paceInfo` is null; pace line not shown in popover

### Requirement: Extra usage credits entry
When `extra_usage.is_enabled` is `true` in the cache, the provider SHALL emit a third entry with id `claude-code-extra`, `windowLabel: "Extra usage"`, `usedCredits` and `limitCredits` populated from the cache, and `usedTokens/limitTokens` both 0.

#### Scenario: Extra usage enabled
- **WHEN** `extra_usage.is_enabled` is true and `monthly_limit` is 2000
- **THEN** entry is emitted with `limitCredits: 2000`, `usedCredits` from `used_credits`

#### Scenario: Extra usage disabled
- **WHEN** `extra_usage.is_enabled` is false
- **THEN** no extra usage entry is emitted
