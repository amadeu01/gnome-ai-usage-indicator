import Adw from 'gi://Adw';
import Gtk from 'gi://Gtk';
import { ExtensionPreferences } from 'resource:///org/gnome/Shell/Extensions/js/extensions/prefs.js';
import { PROVIDER_IDS } from './types.js';

export default class AiUsagePreferences extends ExtensionPreferences {
  override fillPreferencesWindow(window: any): void {
    const settings = this.getSettings();

    const page = new Adw.PreferencesPage({
      title: 'AI Usage Indicator',
      icon_name: 'preferences-system-symbolic',
    });
    window.add(page);

    // ── Providers group ──────────────────────────────────────────────────
    const providersGroup = new Adw.PreferencesGroup({ title: 'Providers' });
    page.add(providersGroup);

    const providers = [
      { id: PROVIDER_IDS.CLAUDE_CODE, label: 'Claude Code', subtitle: 'Reads ~/.claude/statusline-usage-cache.json' },
      { id: PROVIDER_IDS.CODEX, label: 'Codex CLI', subtitle: 'Reads ~/.codex/ usage files' },
      { id: PROVIDER_IDS.ANTHROPIC_API, label: 'Anthropic API', subtitle: 'Fetches from api.anthropic.com (requires API key)' },
    ];

    for (const provider of providers) {
      const row = new Adw.SwitchRow({
        title: provider.label,
        subtitle: provider.subtitle,
      });

      const enabledProviders: string[] = settings.get_strv('enabled-providers');
      row.active = enabledProviders.includes(provider.id);

      row.connect('notify::active', () => {
        const current: string[] = settings.get_strv('enabled-providers');
        if (row.active && !current.includes(provider.id)) {
          settings.set_strv('enabled-providers', [...current, provider.id]);
        } else if (!row.active && current.includes(provider.id)) {
          settings.set_strv('enabled-providers', current.filter((id: string) => id !== provider.id));
        }
      });

      providersGroup.add(row);
    }

    // ── API Keys group ───────────────────────────────────────────────────
    const apiGroup = new Adw.PreferencesGroup({ title: 'API Keys' });
    page.add(apiGroup);

    // Security warning banner
    const banner = new Adw.Banner({
      title: 'API keys are stored in dconf and are not encrypted. Use read-only keys with minimal permissions.',
      revealed: true,
    });
    apiGroup.add(banner);

    // Anthropic API key
    const apiKeyRow = new Adw.PasswordEntryRow({
      title: 'Anthropic API Key',
    });
    apiKeyRow.text = settings.get_string('anthropic-api-key');
    apiKeyRow.connect('changed', () => {
      settings.set_string('anthropic-api-key', apiKeyRow.text);
    });
    apiGroup.add(apiKeyRow);

    // ── Advanced group ───────────────────────────────────────────────────
    const advancedGroup = new Adw.PreferencesGroup({ title: 'Advanced' });
    page.add(advancedGroup);

    const pollRow = new Adw.SpinRow({
      title: 'Poll Interval',
      subtitle: 'Seconds between data refreshes (10–3600)',
      adjustment: new Gtk.Adjustment({
        lower: 10,
        upper: 3600,
        step_increment: 10,
        value: settings.get_int('poll-interval'),
      }),
    });
    pollRow.connect('changed', () => {
      settings.set_int('poll-interval', pollRow.value);
    });
    advancedGroup.add(pollRow);
  }
}
