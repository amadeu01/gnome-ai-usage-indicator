import Soup from 'gi://Soup';
import GLib from 'gi://GLib';
import { ProviderData, PROVIDER_IDS } from '../types.js';

const API_BASE = 'https://api.anthropic.com';

export async function fetchAnthropicApiData(apiKey: string): Promise<ProviderData> {
  if (!apiKey) {
    return {
      id: PROVIDER_IDS.ANTHROPIC_API,
      name: 'Anthropic API',
      usedTokens: 0,
      limitTokens: 100,
      usedCredits: null,
      limitCredits: null,
      resetAt: null,
      error: 'No API key configured',
    };
  }

  return new Promise((resolve) => {
    try {
      const session = new Soup.Session();
      const message = Soup.Message.new('GET', `${API_BASE}/v1/usage`);
      message.request_headers.append('x-api-key', apiKey);
      message.request_headers.append('anthropic-version', '2023-06-01');

      session.send_and_read_async(
        message,
        GLib.PRIORITY_DEFAULT,
        null,
        (sess: any, result: any) => {
          try {
            const bytes = sess.send_and_read_finish(result);
            if (message.status_code === 401) {
              resolve({
                id: PROVIDER_IDS.ANTHROPIC_API,
                name: 'Anthropic API',
                usedTokens: 0,
                limitTokens: 100,
                usedCredits: null,
                limitCredits: null,
                resetAt: null,
                error: 'API error (401)',
              });
              return;
            }
            if (message.status_code !== 200) {
              resolve({
                id: PROVIDER_IDS.ANTHROPIC_API,
                name: 'Anthropic API',
                usedTokens: 0,
                limitTokens: 100,
                usedCredits: null,
                limitCredits: null,
                resetAt: null,
                error: `API error (${message.status_code})`,
              });
              return;
            }

            const decoder = new TextDecoder('utf-8');
            const text = decoder.decode(bytes.get_data());
            const data = JSON.parse(text);

            // Anthropic usage API response shape may vary; handle defensively
            const used = data.input_tokens ?? data.tokens_used ?? 0;
            const limit = data.input_tokens_limit ?? data.tokens_limit ?? 0;
            const pct = limit > 0 ? Math.round((used / limit) * 100) : 0;

            resolve({
              id: PROVIDER_IDS.ANTHROPIC_API,
              name: 'Anthropic API',
              usedTokens: pct,
              limitTokens: 100,
              usedCredits: data.credits_used ?? null,
              limitCredits: data.credits_limit ?? null,
              resetAt: data.reset_at ? new Date(data.reset_at) : null,
              error: null,
            });
          } catch (e) {
            resolve({
              id: PROVIDER_IDS.ANTHROPIC_API,
              name: 'Anthropic API',
              usedTokens: 0,
              limitTokens: 100,
              usedCredits: null,
              limitCredits: null,
              resetAt: null,
              error: 'Parse error',
            });
          }
        }
      );
    } catch (e) {
      resolve({
        id: PROVIDER_IDS.ANTHROPIC_API,
        name: 'Anthropic API',
        usedTokens: 0,
        limitTokens: 100,
        usedCredits: null,
        limitCredits: null,
        resetAt: null,
        error: String(e),
      });
    }
  });
}
