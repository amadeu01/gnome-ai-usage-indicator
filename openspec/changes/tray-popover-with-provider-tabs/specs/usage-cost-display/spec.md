## ADDED Requirements

### Requirement: Cost row shown when data available
When a `ProviderData` entry has `tokens_used` or `cost_usd` populated, the entry section SHALL display a cost row showing token count and/or dollar amount. When both are `None` the cost row SHALL be omitted.

#### Scenario: Both tokens and cost available
- **WHEN** entry has `tokens_used = Some(218_000_000)` and `cost_usd = Some(254.24)`
- **THEN** UI shows "218M tokens · $254.24" in the cost row

#### Scenario: Only tokens available
- **WHEN** entry has `tokens_used = Some(15_000)` and `cost_usd = None`
- **THEN** UI shows "15K tokens" in the cost row

#### Scenario: Neither available
- **WHEN** both `tokens_used` and `cost_usd` are `None`
- **THEN** no cost row is rendered

### Requirement: ProviderData carries cost fields
The `ProviderData` struct SHALL include `tokens_used: Option<u64>` and `cost_usd: Option<f64>`. Provider fetch functions that have no cost information SHALL leave these as `None`.

#### Scenario: Claude Code provider populates fields when cache contains cost data
- **WHEN** the Claude Code cache JSON contains a `cost` object with `total_tokens` and `total_usd`
- **THEN** the returned `ProviderData` entries have `tokens_used` and `cost_usd` set accordingly

#### Scenario: Ollama provider leaves cost fields empty
- **WHEN** Ollama data is fetched
- **THEN** `tokens_used` and `cost_usd` are both `None`
