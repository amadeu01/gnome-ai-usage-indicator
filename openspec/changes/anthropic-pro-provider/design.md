## Context

Claude Code with OAuth authentication (Pro/Max subscription) stores credentials in `~/.claude/.credentials.json`. The OAuth access token can query a dedicated usage endpoint:

```
GET https://api.anthropic.com/api/oauth/usage
Authorization: Bearer <accessToken>
anthropic-beta: oauth-2025-04-20
```

Response shape:
```json
{
  "five_hour": {
    "utilization": 75.0,
    "resets_at": "2026-06-15T18:10:00.439077+00:00"
  },
  "seven_day": {
    "utilization": 55.0,
    "resets_at": "2026-06-18T21:00:00.439098+00:00"
  },
  "extra_usage": {
    "is_enabled": true,
    "monthly_limit": 1700,
    "used_credits": 0.0,
    "currency": "EUR"
  }
}
```

Claude Code itself caches this response at `/tmp/claude/statusline-usage-cache-<hash>.json` where `hash` is SHA256 of the config directory (`~/.claude`), first 8 hex characters, for 60 seconds. The provider should read this cache when fresh to avoid hitting the API rate limit.

## Goals / Non-Goals

**Goals:**
- Auto-discover OAuth token from `~/.claude/.credentials.json` (field: `claudeAiOauth.accessToken`)
- Compute cache path hash: `SHA256($HOME/.claude)` → first 8 chars
- Read cache if fresh (<60s old); otherwise query the API directly
- Return up to 3 ProviderData entries: 5h window, 7d window, extra usage (if enabled)
- Each entry includes utilization %, reset time, window label
- Fail gracefully when credentials file is missing or token is expired

**Non-Goals:**
- Token refresh via `refreshToken` (return error if token expired)
- macOS Keychain or GNOME Keyring token resolution (Linux file only for v1)
- Pace computation (can be added later using the same formula as `claude_code.rs`)
- Reading from the old `statusline-usage-cache.json` path

## Decisions

### D1: Cache-first, API-fallback

Read `/tmp/claude/statusline-usage-cache-<hash>.json` first. If it exists and is < 60s old, use it directly. Otherwise query the API and cache the result.

Rationale: Claude Code already writes this cache. Multiple concurrent Claude Code instances share it. Reading cache avoids rate limiting and is faster.

### D2: Hash computation

```rust
use sha2::{Sha256, Digest};
let hash = hex::encode(&Sha256::digest(config_dir.as_bytes())[..4]); // first 8 hex chars
```

Add `sha2` and `hex` crates to dependencies.

### D3: Token expiry check

The credentials file has `claudeAiOauth.expiresAt` (epoch milliseconds). Compare against `Utc::now().timestamp_millis()`. If expired, return error entry.

### D4: ProviderData mapping

| API field | ProviderData entry |
|-----------|-------------------|
| `five_hour` | `id: "anthropic-sub-5h"`, `window_label: "Session"` |
| `seven_day` | `id: "anthropic-sub-7d"`, `window_label: "Weekly"` |
| `extra_usage` (if `is_enabled`) | `id: "anthropic-sub-extra"`, `window_label: "Extra usage"`, `used_credits`, `limit_credits` |

All share `name: "Anthropic Subscription"`.

### D5: Dependencies

Add to `Cargo.toml`:
- `sha2` (SHA256 for cache hash)
- `hex` (hex encoding for hash display)

`reqwest` is already a dependency.

## Risks / Trade-offs

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Beta API changes | Medium | Medium | The `anthropic-beta` header pins a specific API version. If Anthropic changes the endpoint, the provider returns an error. |
| Token expires during long uptime | Low | Medium | Check `expiresAt` on each fetch. Return error entry: "Token expired — run `claude login`". |
| API rate limiting on cache miss | Low | Low | Cache-first strategy limits API calls to once per 60s across all providers. |
| `sha2`/`hex` dependency bloat | Low | Low | Both are small, widely-used crates with no transitive dependency risk. |

## Files Changed

| File | Change |
|------|--------|
| `Cargo.toml` | Add `sha2`, `hex` dependencies |
| `src/providers/anthropic_subscription.rs` | **New** — token discovery, cache, API fetch, response parsing |
| `src/providers/mod.rs` | Register module + `fetch_all` dispatch |
| `ts-src/src/types.ts` | Add `ANTHROPIC_SUBSCRIPTION` to `PROVIDER_IDS` |
| `ts-src/src/prefs.ts` | Add toggle row |
