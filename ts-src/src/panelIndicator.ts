import St from 'gi://St';
import Gio from 'gi://Gio';
import GObject from 'gi://GObject';
import { PopupBaseMenuItem } from 'resource:///org/gnome/shell/ui/popupMenu.js';
import { Button } from 'resource:///org/gnome/shell/ui/panelMenu.js';
import { buildProviderTile } from './usagePopover.js';
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
  const dot = new St.Widget({
    style_class: 'ai-usage-provider-dot',
  });
  (dot as any).style = `background-color: ${color};`;
  return dot;
}

export const PanelIndicator = GObject.registerClass(
  { GTypeName: 'AiUsagePanelIndicator' },
  class PanelIndicator extends Button {
    private _icon: St.Icon;
    private _dotsBox: St.BoxLayout;
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

      // Rebuild menu content on data change

      // Fetch data when menu opens
      this._openStateId = this.menu.connect('open-state-changed', (_menu: any, open: boolean) => {
        if (open) {
          manager.fetchAll().catch(() => {});
        }
      });

      manager.connect('data-updated', () => {
        this._updateState(manager.data);
        this._rebuildMenu(manager);
      });
    }

    private _rebuildMenu(manager: ProviderManagerType): void {
      // Remove old tile items
      (this.menu as any).removeAll();

      const data = manager.data;
      const sorted = [...data].sort((a: ProviderData, b: ProviderData) => a.name.localeCompare(b.name));

      for (const d of sorted) {
        const tile = buildProviderTile(d);
        const item = new PopupBaseMenuItem({ activate: false });
        item.actor.add_child(tile as any);
        this.menu.addMenuItem(item);
      }

      // Footer
      const lastFetch = manager.lastFetch;
      const diffSecs = lastFetch ? Math.floor((Date.now() - lastFetch.getTime()) / 1000) : -1;
      let footerText: string;
      if (diffSecs < 0) footerText = 'Never updated';
      else if (diffSecs < 60) footerText = 'Updated just now';
      else footerText = `Updated ${Math.floor(diffSecs / 60)} min ago`;

      const footerItem = new PopupBaseMenuItem({ activate: false });
      const footerLabel = new St.Label({ text: footerText, style_class: 'ai-usage-footer', x_expand: true });
      footerItem.actor.add_child(footerLabel);
      this.menu.addMenuItem(footerItem);
    }

    private _updateState(data: ProviderData[]): void {
      if (!this._icon) return;

      // Icon color based on worst utilization
      this._icon.remove_style_class_name('ai-usage-icon-warning');
      this._icon.remove_style_class_name('ai-usage-icon-critical');

      let maxPct = 0;
      for (const d of data) {
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
        if (seen.has(d.name)) continue;
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
      super.destroy();
    }
  },
);

export type PanelIndicatorType = InstanceType<typeof PanelIndicator>;
