## MODIFIED Requirements

### Requirement: Popover shows per-provider usage tiles
The usage popover SHALL display one tile group per enabled provider. Each group SHALL render its `ProviderData` entries as stacked named sections. Entries with a `windowLabel` SHALL render the label as a bold section header. Entries without a `windowLabel` render as before (a single unnamed tile).

#### Scenario: Multiple providers enabled
- **WHEN** user opens the popover with Claude Code and Ollama both enabled
- **THEN** popover shows Claude Code group (with Session / Weekly sections) followed by Ollama group

#### Scenario: No providers enabled
- **WHEN** user opens the popover with all providers disabled
- **THEN** popover shows a single message: "No providers enabled — open Preferences"

#### Scenario: Provider at usage limit
- **WHEN** a provider has used 100% of its limit
- **THEN** its progress bar is full and red; reset countdown is displayed prominently

## ADDED Requirements

### Requirement: Pace line in weekly section
When a `ProviderData` entry includes a non-null `paceInfo`, the tile SHALL show a pace line below the usage text formatted as "Pace: {label} · Lasts to reset".

#### Scenario: Ahead of pace
- **WHEN** weekly entry has `paceInfo.ahead: true` and `paceInfo.label: "Ahead (+30%)"`
- **THEN** tile shows "Pace: Ahead (+30%) · Lasts to reset" in amber text

#### Scenario: Behind pace
- **WHEN** weekly entry has `paceInfo.ahead: false` and `paceInfo.label: "Behind (-20%)"`
- **THEN** tile shows "Pace: Behind (-20%) · Lasts to reset" in muted text

### Requirement: Usage text shows percentage for percentage-based data
When `limitTokens === 100` (percentage-sentinel), the usage text SHALL show `"X% used"` instead of `"X / 100 (X%)"`.

#### Scenario: Percentage-based Claude Code entry
- **WHEN** entry has `usedTokens: 22, limitTokens: 100`
- **THEN** usage text shows "22% used"

### Requirement: Extra usage section shows credits
When an entry has non-zero `limitCredits`, the tile SHALL show `"$usedCredits / $limitCredits"` as the usage text and SHALL NOT show a token-based progress bar.

#### Scenario: Extra usage entry rendered
- **WHEN** entry has `usedCredits: 0, limitCredits: 2000`
- **THEN** tile shows "This month: $0.00 / $2000.00" and "0% used"
