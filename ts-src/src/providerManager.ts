import GObject from 'gi://GObject';
import Gio from 'gi://Gio';
import { ProviderData, PROVIDER_IDS } from './types.js';
import { fetchClaudeCodeData } from './providers/claudeCode.js';
import { fetchCodexData } from './providers/codex.js';
import { fetchAnthropicApiData } from './providers/anthropicApi.js';

export const ProviderManager = GObject.registerClass(
  {
    GTypeName: 'AiUsageProviderManager',
    Signals: {
      'data-updated': {},
    },
  },
  class ProviderManager extends GObject.Object {
    private _settings: any;
    private _data: ProviderData[] = [];
    private _fetching: boolean = false;
    private _lastFetch: Date | null = null;

    constructor(settings: any) {
      super();
      this._settings = settings;
    }

    get data(): ProviderData[] {
      return this._data;
    }

    get lastFetch(): Date | null {
      return this._lastFetch;
    }

    get isFetching(): boolean {
      return this._fetching;
    }

    async fetchAll(): Promise<void> {
      if (this._fetching) return;
      this._fetching = true;

      try {
        const enabledProviders: string[] = this._settings.get_strv('enabled-providers');
        const apiKey: string = this._settings.get_string('anthropic-api-key');
        const results: ProviderData[] = [];

        const fetchTasks: Promise<void>[] = [];

        if (enabledProviders.includes(PROVIDER_IDS.CLAUDE_CODE)) {
          fetchTasks.push(
            Promise.resolve().then(() => {
              try {
                const data = fetchClaudeCodeData();
                results.push(...data);
              } catch (e) {
                results.push({
                  id: PROVIDER_IDS.CLAUDE_CODE,
                  name: 'Claude Code',
                  usedTokens: 0,
                  limitTokens: 100,
                  usedCredits: null,
                  limitCredits: null,
                  resetAt: null,
                  error: String(e),
                });
              }
            })
          );
        }

        if (enabledProviders.includes(PROVIDER_IDS.CODEX)) {
          fetchTasks.push(
            Promise.resolve().then(() => {
              try {
                results.push(fetchCodexData());
              } catch (e) {
                results.push({
                  id: PROVIDER_IDS.CODEX,
                  name: 'Codex',
                  usedTokens: 0,
                  limitTokens: 100,
                  usedCredits: null,
                  limitCredits: null,
                  resetAt: null,
                  error: String(e),
                });
              }
            })
          );
        }

        if (enabledProviders.includes(PROVIDER_IDS.ANTHROPIC_API) && apiKey) {
          fetchTasks.push(
            fetchAnthropicApiData(apiKey)
              .then((d) => { results.push(d); })
              .catch((e) => {
                results.push({
                  id: PROVIDER_IDS.ANTHROPIC_API,
                  name: 'Anthropic API',
                  usedTokens: 0,
                  limitTokens: 100,
                  usedCredits: null,
                  limitCredits: null,
                  resetAt: null,
                  error: String(e),
                });
              })
          );
        }

        await Promise.all(fetchTasks);

        this._data = results;
        this._lastFetch = new Date();
        this.emit('data-updated');
      } finally {
        this._fetching = false;
      }
    }
  }
);

export type ProviderManagerType = InstanceType<typeof ProviderManager>;
