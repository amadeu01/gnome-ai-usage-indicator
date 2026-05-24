## Context

The extension currently reads `~/.claude/statusline-usage-cache.json` and converts `five_hour.utilization` and `seven_day.utilization` into two `ProviderData` entries named "Claude Code (5h)" and "Claude Code (7d)". Both entries store utilization as `usedTokens` with `limitTokens: 100`, making the popover show "22 / 100" — meaningless to the user.

The `extra_usage` field in the cache carries dollar-credit usage but is currently ignored. Ollama has no built-in usage tracking; its `/api/ps` endpoint returns currently-loaded models and VRAM usage, which is useful as a "is it running / what's loaded" status.

## Goals / Non-Goals

**Goals:**
- Render Session (5h) and Weekly (7d) as clearly-labelled sections in the Claude Code tile
- Show pace indicator on Weekly: compares elapsed fraction of the window to usage fraction
- Surface `extra_usage` credits when `is_enabled: true`
- Add Ollama tile showing running models (or "Not running")
- Add `windowLabel` and `paceInfo` fields to `ProviderData`

**Non-Goals:**
- Per-model breakdown (Sonnet/Opus split) — data is sometimes absent and complicates layout
- Remote Ollama instances — localhost only
- Historical usage graphs

## Decisions

### D1: `ProviderData` extension vs separate type
Add optional `windowLabel?: string` and `paceInfo?: { label: string; ahead: boolean } | null` to the existing `ProviderData` interface rather than a new type. Keeps the renderer generic — tiles with a `windowLabel` get a section header; those without don't.

Rejected: separate `WindowedProviderData` — adds discriminated union complexity for a two-field addition.

### D2: Pace calculation
`pace = usedFraction / elapsedFraction` where:
- `usedFraction = utilization / 100`
- `elapsedFraction = (windowDuration - timeToReset) / windowDuration`

If `pace > 1` → "Ahead (+X%)" (amber), `pace < 0.8` → "Behind (-X%)" (muted), otherwise "On track".  
`windowDuration` is 5h for session, 7d for weekly (hardcoded constants matching Claude's windows).

### D3: Ollama fetch via Soup.Session
Use the same `Soup.Session` pattern as the Anthropic API provider — `GET localhost:11434/api/ps` with a short timeout (3s). Returns `{ models: [{ name, size_vram }] }`. If the request fails (connection refused / timeout), the tile shows "Not running". No API key needed.

### D4: Extra usage section
Rendered as a separate `ProviderData` entry with `id: 'claude-code-extra'` when `extra_usage.is_enabled`. Shows `$used / $limit` text and a credit-based progress bar. When disabled, not emitted (no empty tile).

### D5: Tile layout for labelled windows
Each `ProviderData` with a `windowLabel` renders its label as a bold section header above the bar. The tile container becomes a vertical list of such sections. Provider name moves to the top of the whole group (rendered once, not per-window).

The `buildProviderTile` function is replaced by `buildProviderGroup(entries: ProviderData[])` that groups entries sharing the same provider prefix and renders them stacked.

## Risks / Trade-offs

- `elapsedFraction` requires knowing `windowDuration`. These are inferred from the `windowLabel` ("5h" → 5×3600s, "7d" → 7×86400s). If Claude changes their window sizes, pace will silently miscalculate. **Mitigation**: derive elapsed from `(now - (resets_at - windowDuration))` only when both `resets_at` and `windowLabel` are present; fall back to hiding the pace line.

- Ollama `/api/ps` only lists loaded models (those in VRAM), not all installed ones. Users may see "Not running" if no model is loaded. **Mitigation**: label the tile "Ollama (no models loaded)" when the response has an empty `models` array, as opposed to "Not running" for connection failure.

- `Soup.Session` in a GNOME Shell extension shares the session with other network callers. A slow Ollama start can block for the timeout period. **Mitigation**: 3s timeout; run fetch in a `Promise.resolve().then()` so it's non-blocking.
