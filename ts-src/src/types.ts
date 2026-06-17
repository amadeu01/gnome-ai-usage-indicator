export interface PaceInfo {
  label: string;
  ahead: boolean;
}

export interface ProviderData {
  id: string;
  name: string;
  utilization: number;
  usedTokens: number;
  limitTokens: number;
  usedCredits: number | null;
  limitCredits: number | null;
  resetAt: Date | null;
  error: string | null;
  windowLabel: string | null;
  paceInfo: PaceInfo | null;
  meta: string | null;
}

export const PROVIDER_IDS = {
  CLAUDE_CODE: 'claude-code',
  CODEX: 'codex',
  ANTHROPIC_API: 'anthropic-api',
  ANTHROPIC_SUBSCRIPTION: 'anthropic-subscription',
  KIMI_CODE: 'kimi-code',
  DEEPSEEK: 'deepseek',
  GITHUB_COPILOT: 'github-copilot',
} as const;
