export interface PaceInfo {
  label: string;
  ahead: boolean;
}

export interface ProviderData {
  id: string;
  name: string;
  usedTokens: number;
  limitTokens: number;
  usedCredits: number | null;
  limitCredits: number | null;
  resetAt: Date | null;
  error: string | null;
  windowLabel: string | null;
  paceInfo: PaceInfo | null;
}

export const PROVIDER_IDS = {
  CLAUDE_CODE: 'claude-code',
  CODEX: 'codex',
  ANTHROPIC_API: 'anthropic-api',
} as const;

export type ProviderId = typeof PROVIDER_IDS[keyof typeof PROVIDER_IDS];
