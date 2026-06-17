## Why

Users with an Anthropic monthly subscription (Pro, Max) track their usage through Claude Code's OAuth system. The OAuth access token in `~/.claude/.credentials.json` can query `api.anthropic.com/api/oauth/usage` (with the `anthropic-beta: oauth-2025-04-20` header) to get session (5h) and weekly (7d) rate limit utilization, plus extra usage credits.

The existing `claude-code` provider reads a now-defunct `statusline-usage-cache.json` from `~/.claude/`. The new Claude Code stores this data at `/tmp/claude/statusline-usage-cache-<sha256>.json`, but the real win is querying the API directly — giving subscription users a working provider without needing the cache file to exist.

## What Changes

- **Add `anthropic-subscription` provider**: Reads OAuth token from `~/.claude/.credentials.json`, queries the OAuth usage API, and returns session/weekly utilization with reset times
- **Cache-aware**: Checks `/tmp/claude/statusline-usage-cache-<hash>.json` first (SHA256 of config dir, 60s TTL) to avoid rate limiting; falls back to direct API call
- **Zero configuration**: No new config fields — the OAuth token is auto-discovered from the credentials file
- **Toggle in GNOME extension preferences** for enable/disable
- **Register provider ID** in `PROVIDER_IDS`

## Capabilities

### New Capabilities
- `anthropic-subscription-provider`: OAuth-authenticated usage fetch from `api.anthropic.com/api/oauth/usage` returning 5h utilization, 7d utilization, reset times, and extra usage credits

### Modified Capabilities
- `preferences-ui`: New "Anthropic Subscription" toggle row
- `provider-registry`: `fetch_all` dispatches to the new provider

## Impact

- `src/providers/anthropic_subscription.rs`: **New file** — token discovery, cache check, API fetch, response parsing
- `src/providers/mod.rs`: Register module + `fetch_all` dispatch
- `ts-src/src/types.ts`: Add `ANTHROPIC_SUBSCRIPTION` to `PROVIDER_IDS`
- `ts-src/src/prefs.ts`: Add toggle row

## Verification

Tested against live API:
```
$ curl -H "Authorization: Bearer <oauth-token>" \
       -H "anthropic-beta: oauth-2025-04-20" \
       https://api.anthropic.com/api/oauth/usage
→ {"five_hour":{"utilization":75.0,"resets_at":"..."},
   "seven_day":{"utilization":55.0,"resets_at":"..."},
   "extra_usage":{"is_enabled":true,"monthly_limit":1700,"used_credits":0.0,...}}
HTTP 200
```

## Out of Scope

- Token refresh (use refreshToken from credentials if access token is expired; for now, return error)
- macOS Keychain token resolution (Linux credentials file only for v1)
- Pace computation (the API returns raw utilization; pace can be added later)
