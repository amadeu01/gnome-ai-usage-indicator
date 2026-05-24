import GLib from 'gi://GLib';
import Gio from 'gi://Gio';
import { ProviderData, PROVIDER_IDS } from '../types.js';

function directoryExists(path: string): boolean {
  try {
    const file = Gio.File.new_for_path(path);
    return file.query_exists(null);
  } catch {
    return false;
  }
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

export function fetchCodexData(): ProviderData {
  const homeDir = GLib.get_home_dir();
  const codexDir = GLib.build_filename(homeDir, '.codex');

  if (!directoryExists(codexDir)) {
    return {
      id: PROVIDER_IDS.CODEX,
      name: 'Codex',
      usedTokens: 0,
      limitTokens: 100,
      usedCredits: null,
      limitCredits: null,
      resetAt: null,
      error: 'Not installed',
    };
  }

  // Try common Codex usage file locations
  const candidates = [
    GLib.build_filename(codexDir, 'usage.json'),
    GLib.build_filename(codexDir, 'state.json'),
    GLib.build_filename(codexDir, 'config.json'),
  ];

  for (const path of candidates) {
    try {
      const data = readJsonFile(path);
      if (!data) continue;

      // Adapt to whatever schema Codex uses
      if (typeof data.usage_pct === 'number') {
        return {
          id: PROVIDER_IDS.CODEX,
          name: 'Codex',
          usedTokens: Math.round(data.usage_pct),
          limitTokens: 100,
          usedCredits: data.credits_used ?? null,
          limitCredits: data.credits_limit ?? null,
          resetAt: data.resets_at ? new Date(data.resets_at) : null,
          error: null,
        };
      }

      if (typeof data.tokens_used === 'number' && typeof data.tokens_limit === 'number') {
        const pct = Math.round((data.tokens_used / data.tokens_limit) * 100);
        return {
          id: PROVIDER_IDS.CODEX,
          name: 'Codex',
          usedTokens: pct,
          limitTokens: 100,
          usedCredits: null,
          limitCredits: null,
          resetAt: data.reset_at ? new Date(data.reset_at) : null,
          error: null,
        };
      }
    } catch {
      // skip malformed files
    }
  }

  // .codex dir exists but no usage data found
  return {
    id: PROVIDER_IDS.CODEX,
    name: 'Codex',
    usedTokens: 0,
    limitTokens: 100,
    usedCredits: null,
    limitCredits: null,
    resetAt: null,
    error: 'No usage data',
  };
}
