import GLib from 'gi://GLib';
import Gio from 'gi://Gio';
import { ProviderData, PROVIDER_IDS } from '../types.js';

interface UsageCacheEntry {
  utilization: number;
  resets_at: string | null;
}

interface UsageCache {
  five_hour?: UsageCacheEntry | null;
  seven_day?: UsageCacheEntry | null;
  ts?: number;
}

function readJsonFile(path: string): any {
  try {
    const file = Gio.File.new_for_path(path);
    if (!file.query_exists(null)) return null;
    const [ok, contents] = file.load_contents(null);
    if (!ok) return null;
    const decoder = new TextDecoder('utf-8');
    return JSON.parse(decoder.decode(contents));
  } catch {
    return null;
  }
}

function cacheEntryToProviderData(
  id: string,
  name: string,
  entry: UsageCacheEntry | null | undefined
): ProviderData | null {
  if (!entry) return null;
  const pct = Math.round(entry.utilization ?? 0);
  const resetAt = entry.resets_at ? new Date(entry.resets_at) : null;
  return {
    id,
    name,
    usedTokens: pct,
    limitTokens: 100,
    usedCredits: null,
    limitCredits: null,
    resetAt,
    error: null,
  };
}

export function fetchClaudeCodeData(): ProviderData[] {
  try {
    const homeDir = GLib.get_home_dir();
    const cachePath = GLib.build_filename(homeDir, '.claude', 'statusline-usage-cache.json');
    const cache: UsageCache | null = readJsonFile(cachePath);

    if (!cache) {
      return [{
        id: PROVIDER_IDS.CLAUDE_CODE,
        name: 'Claude Code',
        usedTokens: 0,
        limitTokens: 100,
        usedCredits: null,
        limitCredits: null,
        resetAt: null,
        error: 'No data — is Claude Code installed?',
      }];
    }

    const results: ProviderData[] = [];

    const fiveHour = cacheEntryToProviderData(
      PROVIDER_IDS.CLAUDE_CODE + '-5h',
      'Claude Code (5h)',
      cache.five_hour
    );
    if (fiveHour) results.push(fiveHour);

    const sevenDay = cacheEntryToProviderData(
      PROVIDER_IDS.CLAUDE_CODE + '-7d',
      'Claude Code (7d)',
      cache.seven_day
    );
    if (sevenDay) results.push(sevenDay);

    if (results.length === 0) {
      return [{
        id: PROVIDER_IDS.CLAUDE_CODE,
        name: 'Claude Code',
        usedTokens: 0,
        limitTokens: 100,
        usedCredits: null,
        limitCredits: null,
        resetAt: null,
        error: 'No usage windows found',
      }];
    }

    return results;
  } catch (e) {
    return [{
      id: PROVIDER_IDS.CLAUDE_CODE,
      name: 'Claude Code',
      usedTokens: 0,
      limitTokens: 100,
      usedCredits: null,
      limitCredits: null,
      resetAt: null,
      error: String(e),
    }];
  }
}
