## Why

The current implementation tracks Claude Code usage from local cache and monitors local Ollama VRAM. However, the real need is tracking **cloud AI provider usage** — token consumption, cost, and rate limits across multiple providers (Ollama Cloud, OpenAI Codex, Claude Code) with session and weekly windows, reset times, and historical usage graphs.

Ollama Cloud and OpenAI Codex don't expose usage data via official APIs, requiring cookie-based web scraping. Users need visibility into their AI spending and limits without manually checking multiple dashboards.

## What Changes

- **Reframe Ollama provider**: From "local VRAM monitor" → "Ollama Cloud usage tracker" via cookie-based scraping of `ollama.com/settings`
- **Add OpenAI Codex provider**: OAuth-based fetching from `chatgpt.com/backend-api/wham/usage` (start with OAuth, add CLI RPC + web later)
- **Keep Claude Code provider**: Already reads local JSON cache correctly
- **Add usage history window**: Separate GTK4 window showing historical trend graphs (daily for 7 days, hourly for current window) using `plotters-rs`
- **Add SQLite persistence**: Store historical snapshots for trend visualization
- **Provider selection**: Users enable/disable providers via TOML config

## Capabilities

### New Capabilities
- `ollama-cloud-provider`: Cookie-based HTML scraping of Ollama Cloud settings page; parses session/weekly usage %, reset times, plan tier
- `openai-codex-provider`: OAuth token auth fetching Codex usage from ChatGPT backend API; parses rate limits, credits, usage breakdown
- `usage-history-window`: Separate GTK4 window with plotters-rs graphs showing historical usage trends (daily + hourly granularity)
- `sqlite-history-store`: Local SQLite database storing periodic usage snapshots for trend analysis

### Modified Capabilities
- `provider-config`: TOML config extended with cookie paths, OAuth tokens, provider enable/disable flags
- `usage-popover`: Enhanced to show provider-specific metadata (plan badges, reset countdowns, pace indicators)

## Impact

- `src/providers/ollama.rs`: Complete rewrite — from localhost polling to cookie-based web scraping
- `src/providers/codex.rs`: New file — OAuth auth, API fetching, response parsing
- `src/providers/claude_code.rs`: Minor updates (already functional)
- `src/history/`: New module — SQLite schema, insert/query functions
- `src/ui/history_window.rs`: New file — GTK4 window with plotters-rs graph rendering
- `src/config.rs`: Extended with OAuth tokens, cookie sources, history retention settings
- `Cargo.toml`: Add `plotters`, `rusqlite`, `reqwest` (cookie support), `keyring` (optional, for token storage)

## Dependencies

| Provider | Data Source | Auth Method | Stability |
|----------|-------------|-------------|-----------|
| Claude Code | `~/.claude/statusline-usage-cache.json` | None (file read) | ✓✓✓ Stable |
| Ollama Cloud | `ollama.com/settings` (HTML scrape) | Browser cookie import | ⚠️ Fragile (HTML may change) |
| OpenAI Codex | `chatgpt.com/backend-api/wham/usage` | OAuth token from `~/.codex/auth.json` | ✓✓ Semi-stable |

## Out of Scope (Future Phases)

- Codex CLI RPC fallback (phase 2)
- Codex web dashboard scraping (phase 2)
- GitHub Copilot integration (different data model — daily metrics, not session/weekly)
- Real-time token streaming (polling-based only)
- Multi-account support (one account per provider for now)
