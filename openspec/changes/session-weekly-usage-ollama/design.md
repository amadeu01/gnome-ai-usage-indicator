## Context

The extension is written in Rust (migrated from TypeScript). It currently tracks Claude Code usage from a local JSON cache and monitors local Ollama VRAM. The real use case is tracking **cloud AI usage** — token consumption, cost, and rate limits across Ollama Cloud, OpenAI Codex, and Claude Code.

Ollama Cloud and OpenAI Codex don't expose usage data via official APIs. Ollama has open feature requests (#15132, #15663) but no endpoints yet. OpenAI Codex exposes usage via OAuth-protected backend APIs. Both require cookie-based or token-based auth to scrape web dashboards.

Historical usage tracking requires local persistence. Users want to see trends: "Am I using more this week than last week?" and "Where am I in the current window?"

## Goals / Non-Goals

**Goals:**
- Track session (5h) and weekly (7d) usage for Claude Code, Ollama Cloud, OpenAI Codex
- Show pace indicators: ahead/behind/on-track relative to window elapsed time
- Display reset countdowns: "Resets in 3h 24m"
- Store historical snapshots in SQLite for trend visualization
- Render usage history graphs: daily trends (7 days) + hourly progress (current window)
- Support multiple providers simultaneously (user-selectable in config)
- Cookie-based auth for Ollama Cloud (browser cookie import)
- OAuth-based auth for OpenAI Codex (read from `~/.codex/auth.json`)

**Non-Goals:**
- GitHub Copilot integration (different data model — daily metrics, not session/weekly limits)
- Codex CLI RPC or web scraping (phase 2)
- Real-time streaming (polling every 60s is sufficient)
- Multi-account per provider (one account per provider for now)
- Team/enterprise metrics (individual usage only)

## Decisions

### D1: Provider Architecture — Trait-Based Polymorphism

Use a `Provider` trait with async `fetch_usage()` method. Each provider implements the trait:

```rust
#[async_trait]
pub trait Provider {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    async fn fetch_usage(&self) -> Result<ProviderData>;
}

pub struct OllamaCloudProvider {
    cookie: String,
}

pub struct CodexOAuthProvider {
    auth_path: PathBuf,
}

pub struct ClaudeCodeProvider {
    cache_path: PathBuf,
}
```

**Rejected:** Enum-based dispatch — harder to extend with new providers.

### D2: Cookie Auth Strategy — Manual Import Only

Users copy cookies from browser dev tools and paste into config file. No automatic browser cookie extraction (too platform-specific, fragile).

Config format:
```toml
[providers.ollama]
enabled = true
cookie = "session_id=abc123; ..."

[providers.codex]
enabled = true
# OAuth token read automatically from ~/.codex/auth.json
```

**Rejected:** Automatic browser cookie extraction (CodexBar does this for Safari/Chrome/Firefox on macOS) — too complex for v1, platform-specific.

### D3: OAuth Token Refresh — Lazy Refresh

Read Codex OAuth tokens from `~/.codex/auth.json`. If token is expired (check `last_refresh` timestamp), trigger refresh flow (phase 2 — for now, assume token is valid or user refreshes manually).

### D4: SQLite Schema — Simple Snapshot Table

```sql
CREATE TABLE usage_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider_id TEXT NOT NULL,
    window_type TEXT NOT NULL,  -- 'session' or 'weekly'
    utilization REAL NOT NULL,
    tokens_used INTEGER,
    cost_usd REAL,
    reset_at TEXT NOT NULL,
    captured_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_provider_window ON usage_snapshots(provider_id, window_type, captured_at);
```

**Rejected:** Complex normalized schema — overkill for simple time-series snapshots.

### D5: Graph Rendering — Plotters-Rs with GTK4 Integration

Use `plotters` crate for graph rendering. Render to bitmap in memory, display in `gtk4::Picture` widget.

```rust
use plotters::prelude::*;

fn render_usage_graph(data: &[UsagePoint]) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    {
        let root = BitMapBackend::new(&mut buffer, (400, 300)).into_drawing_area();
        // ... build chart ...
    }
    Ok(buffer) // PNG bytes
}
```

Graph types:
- **Daily trend**: Last 7 days, one point per day, line chart
- **Hourly progress**: Current window (5h or 7d), one point per hour, area chart

**Rejected:** 
- `gtk4::Snapshot` immediate mode — too low-level, reinventing charting
- ASCII-style bars — not professional enough for usage data

### D6: Polling Strategy — Single Background Task

One tokio task polls all enabled providers every 60 seconds (configurable). Results stored in SQLite and broadcast to UI via async channel.

```rust
async fn polling_loop(config: Config, tx: Sender<Vec<ProviderData>>) {
    let mut interval = interval(Duration::from_secs(config.poll_interval_secs));
    loop {
        interval.tick().await;
        let data = fetch_all_providers(&config).await;
        store_snapshots(&data).await;
        tx.send(data).await?;
    }
}
```

**Rejected:** Per-provider polling intervals — adds complexity, 60s is fine for all.

### D7: Error Handling — Graceful Degradation

If a provider fails (cookie expired, network error), show error state in UI but don't crash. Other providers continue working.

```rust
pub enum ProviderData {
    Success { ... },
    Error { message: String, provider_id: String },
}
```

### D8: Config Location — XDG Base Dir

Config file: `~/.config/ai-usage-indicator/config.toml`
SQLite DB: `~/.local/share/ai-usage-indicator/history.db`

Follows XDG Base Directory spec.

**Rejected:** All in `~/.config` — mixing config and data is messy.

## Risks / Trade-offs

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Ollama changes HTML structure | High | Medium | Clear error message: "Ollama changed their UI — please update cookie or check for app update". Parser isolated in `ollama_parser.rs` for easy updates. |
| OAuth token expires | Medium | Low | Detect 401 responses, show "Token expired — please refresh Codex" message. Manual refresh in UI (phase 2). |
| SQLite write contention | Low | Low | Single writer (polling task), multiple readers (UI). Use `rusqlite` with `PRAGMA journal_mode=WAL`. |
| Plotters bitmap scaling | Medium | Low | Test on HiDPI displays. Use `scale_factor` from GTK4 to adjust bitmap size. |
| Cookie import friction | High | Medium | Document clear setup steps. Consider browser extension for one-click copy (future). |
| Rate limiting from providers | Low | Medium | Add exponential backoff. Respect `Retry-After` headers if present. |

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         SYSTEM ARCHITECTURE                              │
└─────────────────────────────────────────────────────────────────────────┘

┌──────────────┐
│   Config     │  ~/.config/ai-usage-indicator/config.toml
│   (TOML)     │  • Provider enable/disable
└──────┬───────┘  • Cookie strings, OAuth paths
       │          • Poll interval, history retention
       │
       ▼
┌──────────────────────────────────────────────────────────────────────────┐
│                        Provider Manager                                   │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────┐  │
│  │ Claude Code     │  │ Ollama Cloud    │  │ OpenAI Codex            │  │
│  │                 │  │                 │  │                         │  │
│  │ • File read     │  │ • Cookie auth   │  │ • OAuth token           │  │
│  │ • JSON parse    │  │ • HTML scrape   │  │ • API fetch             │  │
│  │ • Stable        │  │ • Fragile       │  │ • Semi-stable           │  │
│  └───────┬─────────┘  └───────┬─────────┘  └───────────┬─────────────┘  │
└──────────┼────────────────────┼────────────────────────┼────────────────┘
           │                    │                        │
           └────────────────────┴────────────────────────┘
                                │
                                ▼
                   ┌─────────────────────────┐
                   │  Vec<ProviderData>      │
                   │  (unified struct)       │
                   └───────────┬─────────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
              ▼                ▼                ▼
     ┌────────────────┐ ┌────────────────┐ ┌────────────────┐
     │  SQLite Store  │ │  System Tray   │ │   Popover      │
     │                │ │  Indicator     │ │                │
     │ • Insert snap  │ │ • Aggregate    │ │ • Per-provider │
     │ • Query trends │ │   summary      │ │   cards        │
     └───────┬────────┘ └────────────────┘ └───────┬────────┘
             │                                     │
             ▼                                     ▼
     ┌────────────────┐                   ┌────────────────┐
     │ History Window │                   │  Provider Card │
     │                │                   │                │
     │ • Plotters-rs  │                   │ • Session/     │
     │ • Daily trend  │                   │   Weekly       │
     │ • Hourly prog  │                   │ • Pace         │
     │ • 7 days       │                   │ • Reset        │
     └────────────────┘                   └────────────────┘
```

## Data Flow

```
1. FETCH (every 60s)
   ┌──────────────────────────────────────────────────────────────┐
   │  Provider Manager                                            │
   │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
   │  │  Claude     │  │  Ollama     │  │   Codex     │          │
   │  │  Code       │  │  Cloud      │  │   OAuth     │          │
   │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘          │
   │         │                │                │                  │
   │         ▼                ▼                ▼                  │
   │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
   │  │ Read JSON   │  │ GET /set-   │  │ GET /back-  │          │
   │  │ file        │  │ tings HTML  │  │ end-api/    │          │
   │  │             │  │ + parse     │  │ wham/usage  │          │
   │  └─────────────┘  └─────────────┘  └─────────────┘          │
   └──────────────────────────────────────────────────────────────┘

2. STORE (async, non-blocking)
   ┌──────────────────────────────────────────────────────────────┐
   │  SQLite: INSERT INTO usage_snapshots ...                     │
   │  • One row per provider per window (session + weekly)        │
   │  • captured_at = now()                                       │
   └──────────────────────────────────────────────────────────────┘

3. BROADCAST (via async-channel)
   ┌──────────────────────────────────────────────────────────────┐
   │  UI Components (all listen to same channel)                  │
   │  • System tray: update indicator icon/tooltip                │
   │  • Popover: refresh provider cards                           │
   │  • History window: if open, update graphs                    │
   └──────────────────────────────────────────────────────────────┘

4. RENDER GRAPHS (on demand, when history window opens)
   ┌──────────────────────────────────────────────────────────────┐
   │  Query SQLite: SELECT * FROM usage_snapshots                 │
   │    WHERE provider_id = ? AND captured_at >= ?                │
   │                                                              │
   │  plotters-rs:                                                │
   │  • Daily: GROUP BY DATE(captured_at), last value per day     │
   │  • Hourly: All points in current window, interpolate         │
   └──────────────────────────────────────────────────────────────┘
```

## UI Mockups

### Popover (Enhanced)

```
┌─────────────────────────────────────────────────────────────┐
│  AI Usage                                    [Settings] [×] │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Claude Code                                                │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Session                          45% used            │  │
│  │  ████████████████░░░░░░░░░░░░░░░░░░░░                 │  │
│  │  Pace: On track · Resets in 2h 15m                    │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Weekly                           28% used            │  │
│  │  ██████████░░░░░░░░░░░░░░░░░░░░░░░░░░                 │  │
│  │  Pace: Behind (-12%) · Resets in 5d 18h               │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  Ollama Cloud                          [Pro Plan]           │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Session                          67% used            │  │
│  │  ████████████████████████░░░░░░░░░░                   │  │
│  │  Pace: Ahead (+23%) · Resets in 1h 42m                │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  OpenAI Codex                                               │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Session                          12% used            │  │
│  │  ████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░                 │  │
│  │  Credits: $4.50 / $50.00                              │  │
│  │  Resets in 4h 33m                                     │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  [View Usage History]                                       │
└─────────────────────────────────────────────────────────────┘
```

### History Window

```
┌─────────────────────────────────────────────────────────────┐
│  Usage History                                [─] [□] [×]   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Provider: [Claude Code ▼]   Window: [Weekly ▼]             │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Weekly Usage - Last 7 Days                           │  │
│  │                                                       │  │
│  │  100% ┤                                               │  │
│  │   80% ┤        ╱╲           ╱╲                        │  │
│  │   60% ┤   ╱╲  ╱  ╲   ╱╲   ╱  ╲   ╱╲                  │  │
│  │   40% ┤  ╱  ╲╱    ╲ ╱  ╲ ╱    ╲ ╱  ╲   ╱╲            │  │
│  │   20% ┤ ╱          ╱    ╱      ╲╱    ╲ ╱  ╲           │  │
│  │    0% ┼───────────────────────────────────────────    │  │
│  │       Mon  Tue  Wed  Thu  Fri  Sat  Sun               │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Current Week Progress (Hourly)                       │  │
│  │                                                       │  │
│  │  100% ┤                                               │  │
│  │   80% ┤                                    ╱╲         │  │
│  │   60% ┤                          ╱╲   ╱╲ ╱  ╲        │  │
│  │   40% ┤                ╱╲   ╱╲ ╱  ╲ ╱  ╱    ╲       │  │
│  │   20% ┤      ╱╲   ╱╲ ╱  ╲ ╱  ╱                      │  │
│  │    0% ┼───────────────────────────────────────────    │  │
│  │       Mon 00 06 12 18 00 06 12 18 00 06 12 18        │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  Legend:                                                    │
│    ████ Session usage    ──── Weekly pace                  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```
