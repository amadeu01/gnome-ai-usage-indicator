import St from 'gi://St';
import Gio from 'gi://Gio';
import GObject from 'gi://GObject';
import { Button } from 'resource:///org/gnome/shell/ui/panelMenu.js';
import { PopupMenuSection } from 'resource:///org/gnome/shell/ui/popupMenu.js';
import { UsagePopover } from './usagePopover.js';
import { ProviderData } from './types.js';
import type { ProviderManagerType } from './providerManager.js';

const PROVIDER_COLORS: Record<string, string> = {
  'Anthropic Subscription': '#d4a574',
  'Kimi Code': '#7b68ee',
  DeepSeek: '#4fc3f7',
  'Claude Code': '#d4a574',
  Codex: '#4fc3f7',
  'Anthropic API': '#d4a574',
};

function providerDot(color: string): St.Widget {
  return new St.Widget({
    style_class: 'ai-usage-provider-dot',
    style: `background-color: ${color};`,
  });
}

export const PanelIndicator = GObject.registerClass(
  { GTypeName: 'AiUsagePanelIndicator' },
  class PanelIndicator extends Button {
    private _icon: St.Icon;
    private _dotsBox: St.BoxLayout;
    private _popoverSection: any;
    private _popoverContent: InstanceType<typeof UsagePopover> | null = null;
    private _openStateId: number = 0;

    constructor(manager: ProviderManagerType, extensionPath: string) {
      super(0.0, 'AI Usage Indicator', false);

      const container = new St.BoxLayout({ style_class: 'ai-usage-panel-box' });

      this._icon = new St.Icon({
        gicon: Gio.icon_new_for_string(`${extensionPath}/icons/ai-usage-indicator-symbolic.svg`),
        style_class: 'system-status-icon ai-usage-icon',
      });
      container.add_child(this._icon);

      this._dotsBox = new St.BoxLayout({ style_class: 'ai-usage-dots-box' });
      container.add_child(this._dotsBox);

      this.add_child(container);

      this._popoverSection = new PopupMenuSection();
      this.menu.addMenuItem(this._popoverSection);

      this._popoverContent = new (UsagePopover as any)(manager);

      this._popoverContent.buildUI(this._popoverSection.box as St.BoxLayout);

      this._openStateId = this.menu.connect('open-state-changed', (_menu: any, open: boolean) => {
        if (open) {
          manager.fetchAll().catch(() => {});
        }
      });

      manager.connect('data-updated', () => {
        this._updateState(manager.data);
      });
    }

    private _updateState(data: ProviderData[]): void {
      if (!this._icon) return;

      // Icon color based on worst utilization
      this._icon.remove_style_class_name('ai-usage-icon-warning');
      this._icon.remove_style_class_name('ai-usage-icon-critical');

      let maxPct = 0;
      for (const d of data) {
        if (d.error) continue;
        const pct = d.utilization > 0 ? Math.round(d.utilization) : 0;
        maxPct = Math.max(maxPct, pct);
      }

      if (maxPct >= 95) {
        this._icon.add_style_class_name('ai-usage-icon-critical');
      } else if (maxPct >= 80) {
        this._icon.add_style_class_name('ai-usage-icon-warning');
      }

      // Per-provider colored dots
      this._dotsBox.destroy_all_children();

      const seen = new Set<string>();
      for (const d of data) {
        if (d.error || seen.has(d.name)) continue;
        seen.add(d.name);

        const color = PROVIDER_COLORS[d.name] || '#888888';
        this._dotsBox.add_child(providerDot(color));
      }
    }

    override destroy(): void {
      if (this._openStateId) {
        this.menu.disconnect(this._openStateId);
        this._openStateId = 0;
      }
      if (this._popoverContent) {
        (this._popoverContent as any).destroy();
        this._popoverContent = null;
      }
      super.destroy();
    }
  },
);

export type PanelIndicatorType = InstanceType<typeof PanelIndicator>;
