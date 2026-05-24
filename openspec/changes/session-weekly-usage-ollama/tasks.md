## 1. Types & Schema

- [ ] 1.1 Add `windowLabel?: string` to `ProviderData` interface in `src/types.ts`
- [ ] 1.2 Add `paceInfo?: { label: string; ahead: boolean } | null` to `ProviderData`
- [ ] 1.3 Add `meta?: string | null` to `ProviderData` (for Ollama model names string)
- [ ] 1.4 Add `ollama-host` key (string, default `http://localhost:11434`) to GSettings schema XML
- [ ] 1.5 Add `PROVIDER_IDS.OLLAMA = 'ollama'` to `src/types.ts`

## 2. Claude Code Provider — Window Parsing

- [ ] 2.1 Add pace calculation helper: given `utilization` and `resets_at` + `windowSeconds`, return `paceInfo`
- [ ] 2.2 Update `cacheEntryToProviderData` to accept a `windowLabel` param and set it on the result
- [ ] 2.3 Pass `windowLabel: 'Session'` and window duration `5 * 3600` when building the `five_hour` entry
- [ ] 2.4 Pass `windowLabel: 'Weekly'` and window duration `7 * 86400` when building the `seven_day` entry; compute and attach `paceInfo`
- [ ] 2.5 Parse `extra_usage` field: when `is_enabled`, emit a `claude-code-extra` entry with `windowLabel: 'Extra usage'`, `usedCredits`, `limitCredits`
- [ ] 2.6 Add ambient.d.ts type for `Gio.icon_new_for_string` return (`Icon`) used in tile rendering

## 3. Ollama Provider

- [ ] 3.1 Create `src/providers/ollama.ts` with `fetchOllamaData(host: string): Promise<ProviderData>`
- [ ] 3.2 Use `Soup.Session` GET `{host}/api/ps` with 3-second timeout
- [ ] 3.3 Parse response: map `models[]` to `meta` string (`"name (X.X GB)"` per model, comma-joined); empty array → `meta: 'No models loaded'`
- [ ] 3.4 On connection failure/timeout: return entry with `error: 'Not running'`, `meta: null`
- [ ] 3.5 Set `usedTokens: 0, limitTokens: 0, resetAt: null` on all Ollama entries

## 4. Provider Manager

- [ ] 4.1 Import and register `fetchOllamaData` in `src/providerManager.ts`
- [ ] 4.2 Read `ollama-host` from GSettings; pass to `fetchOllamaData`
- [ ] 4.3 Add Ollama fetch to `fetchTasks` when `PROVIDER_IDS.OLLAMA` is in `enabled-providers`

## 5. Usage Popover — Tile Renderer

- [ ] 5.1 Add `buildProviderGroup(entries: ProviderData[]): St.BoxLayout` function that renders all entries for one provider stacked, with the provider name as the group header
- [ ] 5.2 When an entry has `windowLabel`, render it as a bold section header (`St.Label`, `style_class: 'ai-usage-window-label'`) above the bar
- [ ] 5.3 Update usage text: when `limitTokens === 100`, show `"X% used"`; when `limitCredits > 0`, show `"$X.XX / $X.XX"`; else show token count ratio
- [ ] 5.4 Add pace line below usage text when `paceInfo` is non-null: `"Pace: {label} · Lasts to reset"`, CSS class `ai-usage-pace-ahead` or `ai-usage-pace-behind`
- [ ] 5.5 For Ollama entries (no progress bar): skip bar rendering; show `meta` string as a `St.Label`
- [ ] 5.6 Replace `buildProviderTile` calls in `_refresh()` with `buildProviderGroup` — group entries by provider ID prefix before rendering
- [ ] 5.7 Add CSS rules for `.ai-usage-window-label`, `.ai-usage-pace-ahead`, `.ai-usage-pace-behind` to `stylesheet.css`

## 6. Preferences

- [ ] 6.1 Add Ollama `Adw.SwitchRow` to providers group in `src/prefs.ts`
- [ ] 6.2 Add Ollama host `Adw.EntryRow` below the Ollama switch (shown always, not just when enabled)
- [ ] 6.3 Wire `ollama-host` EntryRow to GSettings key

## 7. Build & Verify

- [ ] 7.1 Run `make build` — confirm TypeScript compiles clean
- [ ] 7.2 Run `make install` — confirm extension files updated
- [ ] 7.3 Reload extension — confirm Session and Weekly sections appear in Claude Code tile
- [ ] 7.4 Verify pace line shows on Weekly section
- [ ] 7.5 Start Ollama (`ollama serve`) — verify Ollama tile shows loaded models
- [ ] 7.6 Stop Ollama — verify tile shows "Not running"
- [ ] 7.7 Open preferences — verify Ollama rows render; set custom host; confirm it persists
