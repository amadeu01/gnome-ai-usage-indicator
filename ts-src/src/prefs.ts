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
      {
        id: PROVIDER_IDS.CLAUDE_CODE,
        label: 'Claude Code',
        subtitle: 'Reads ~/.claude/statusline-usage-cache.json',
      },
      { id: PROVIDER_IDS.CODEX, label: 'Codex CLI', subtitle: 'Reads ~/.codex/ usage files' },
      {
        id: PROVIDER_IDS.ANTHROPIC_API,
        label: 'Anthropic API',
        subtitle: 'Fetches from api.anthropic.com',
      },
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
          settings.set_strv(
            'enabled-providers',
            current.filter((id: string) => id !== provider.id)
          );
        }
      });

      providersGroup.add(row);
    }

    // ── Configuration group ──────────────────────────────────────────────
    const configGroup = new Adw.PreferencesGroup({ title: 'Configuration' });
    page.add(configGroup);

    const configRow = new Adw.ActionRow({
      title: 'Daemon Config File',
      subtitle: '~/.config/ai-usage-indicator/config.toml',
      activatable: false,
    });
    configRow.add_suffix(new Gtk.Image({ icon_name: 'document-edit-symbolic' }));
    configGroup.add(configRow);

    const apiKeyNote = new Adw.ActionRow({
      title: 'API Keys',
      subtitle: 'Set anthropic_api_key in the config file above. Keys are never stored in dconf.',
      activatable: false,
    });
    configGroup.add(apiKeyNote);

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
