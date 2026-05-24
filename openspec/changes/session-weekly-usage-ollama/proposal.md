## Why

The current popover shows Claude Code usage as opaque "22 / 100" numbers with no context for what time window they cover — session (5h) vs weekly (7d) are separate limits with different reset times but appear as disconnected tiles. Ollama is a widely-used local LLM runner with no provider tile at all.

## What Changes

- Restructure Claude Code tiles to label usage windows explicitly: **Session (5h)** and **Weekly (7d)** as distinct named sections, each showing percentage used + reset countdown
- Display extra usage credits from the cache (`extra_usage` field) when enabled
- Add **Ollama provider**: poll `http://localhost:11434/api/ps` to detect running models; show which models are loaded and total VRAM used
- The popover layout follows the screenshot reference: each provider section has a header, a progress bar, `X% used` text, and `Resets in Xd Xh` on the right
- Weekly section shows a **Pace** line: whether consumption is ahead/behind a linear pace toward the reset

## Capabilities

### New Capabilities
- `claude-code-usage-windows`: Session (5h) and Weekly (7d) shown as distinct labeled sections with pace indicator on weekly
- `ollama-provider`: Local Ollama detection via REST API; shows running models, not rate-limit data (Ollama has none)

### Modified Capabilities
- `usage-popover`: Layout updated — each provider renders named usage-window sections instead of a single aggregated tile; extra usage section added when credits are enabled
- `provider-data-fetch`: `ProviderData` extended with `windowLabel` and `paceInfo` fields; Ollama fetch added

## Impact

- `src/providers/claudeCode.ts`: parse `extra_usage` field; attach `windowLabel` to each entry
- `src/providers/ollama.ts`: new file; `Soup.Session` GET to `localhost:11434/api/ps`
- `src/usagePopover.ts`: tile renderer updated for labeled windows + pace line
- `src/types.ts`: `ProviderData` interface extended
- `src/prefs.ts`: Ollama toggle row added to providers group
- GSettings schema: add `ollama-host` (string, default `http://localhost:11434`)
