import St from 'gi://St';
import Gio from 'gi://Gio';
import GObject from 'gi://GObject';
import { Button } from 'resource:///org/gnome/shell/ui/panelMenu.js';
import { PopupMenuSection } from 'resource:///org/gnome/shell/ui/popupMenu.js';
import { UsagePopover } from './usagePopover.js';
import { ProviderData } from './types.js';
import type { ProviderManagerType } from './providerManager.js';

export const PanelIndicator = GObject.registerClass(
  { GTypeName: 'AiUsagePanelIndicator' },
  class PanelIndicator extends Button {
    private _icon: St.Icon;
    private _popoverSection: any;
    private _popoverContent: InstanceType<typeof UsagePopover> | null = null;
    private _openStateId: number = 0;

    constructor(manager: ProviderManagerType, extensionPath: string) {
      super(0.0, 'AI Usage Indicator', false);

      this._icon = new St.Icon({
        gicon: Gio.icon_new_for_string(`${extensionPath}/icons/ai-usage-indicator-symbolic.svg`),
        style_class: 'system-status-icon ai-usage-icon',
      });
      this.add_child(this._icon);

      this._popoverSection = new PopupMenuSection();
      this.menu.addMenuItem(this._popoverSection);

      this._popoverContent = new (UsagePopover as any)(this._popoverSection, manager);

      this._openStateId = this.menu.connect('open-state-changed', (_menu: any, open: boolean) => {
        if (open) {
          manager.fetchAll().catch(() => {});
        }
      });

      manager.connect('data-updated', () => {
        this._updateIconColor(manager.data);
      });
    }

    private _updateIconColor(data: ProviderData[]): void {
      if (!this._icon) return;
      this._icon.remove_style_class_name('ai-usage-icon-warning');
      this._icon.remove_style_class_name('ai-usage-icon-critical');

      const maxPct = data.reduce((max, d) => {
        if (d.error) return max;
        const pct = d.limitTokens > 0 ? Math.round((d.usedTokens / d.limitTokens) * 100) : 0;
        return Math.max(max, pct);
      }, 0);

      if (maxPct >= 95) {
        this._icon.add_style_class_name('ai-usage-icon-critical');
      } else if (maxPct >= 80) {
        this._icon.add_style_class_name('ai-usage-icon-warning');
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
  }
);

export type PanelIndicatorType = InstanceType<typeof PanelIndicator>;
