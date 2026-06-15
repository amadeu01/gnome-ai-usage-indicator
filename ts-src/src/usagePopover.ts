import St from 'gi://St';
import GLib from 'gi://GLib';
import GObject from 'gi://GObject';
import { ProviderData } from './types.js';
import type { ProviderManagerType } from './providerManager.js';

function formatResetCountdown(resetAt: Date | null): string {
  if (!resetAt) return '';
  const now = new Date();
  const diffMs = resetAt.getTime() - now.getTime();
  if (diffMs <= 0) return 'Resetting\u2026';
  const totalSecs = Math.floor(diffMs / 1000);
  const days = Math.floor(totalSecs / 86400);
  const hours = Math.floor((totalSecs % 86400) / 3600);
  const mins = Math.floor((totalSecs % 3600) / 60);
  if (days > 0) return `Resets in ${days}d ${hours}h`;
  if (hours > 0) return `Resets in ${hours}h ${mins}m`;
  return `Resets in ${mins}m`;
}

function getUsageClass(pct: number): string {
  if (pct >= 95) return 'usage-bar-critical';
  if (pct >= 80) return 'usage-bar-warning';
  return 'usage-bar-ok';
}

function formatLastUpdated(lastFetch: Date | null): { text: string; stale: boolean } {
  if (!lastFetch) return { text: 'Never updated', stale: true };
  const diffSecs = Math.floor((Date.now() - lastFetch.getTime()) / 1000);
  const stale = diffSecs > 300;
  if (diffSecs < 60) return { text: 'Updated just now', stale };
  const mins = Math.floor(diffSecs / 60);
  return { text: `Updated ${mins} min ago`, stale };
}

function buildProviderTile(data: ProviderData): St.BoxLayout {
  const tile = new St.BoxLayout({
    style_class: 'ai-usage-tile',
    vertical: true,
  });

  const header = new St.BoxLayout({ style_class: 'ai-usage-tile-header' });

  const nameLabel = new St.Label({
    text: data.windowLabel ? `${data.name} \u2014 ${data.windowLabel}` : data.name,
    style_class: 'ai-usage-provider-name',
    x_expand: true,
  });
  header.add_child(nameLabel);

  // Prefer provider-reported utilization; fall back to token-based computation
  const pct =
    data.utilization > 0
      ? Math.round(data.utilization)
      : data.limitTokens > 0
        ? Math.min(100, Math.round((data.usedTokens / data.limitTokens) * 100))
        : 0;

  // Show meta (e.g. balance) when usage is 0
  if (pct === 0 && data.meta) {
    const metaLabel = new St.Label({
      text: data.meta,
      style_class: 'ai-usage-pct-label',
    });
    header.add_child(metaLabel);
  } else {
    const pctLabel = new St.Label({
      text: `${pct}%`,
      style_class: 'ai-usage-pct-label',
    });
    header.add_child(pctLabel);
  }
  tile.add_child(header);

  if (data.error) {
    const errorLabel = new St.Label({
      text: data.error,
      style_class: 'ai-usage-error',
    });
    tile.add_child(errorLabel);
    return tile;
  }

  // Progress bar — only when utilization > 0
  if (pct > 0) {
    const barTrack = new St.Widget({ style_class: 'ai-usage-bar-track', x_expand: true });
    const barFill = new St.Widget({
      style_class: `ai-usage-bar-fill ${getUsageClass(pct)}`,
      style: `width: ${pct}%;`,
    });
    barTrack.add_child(barFill);
    tile.add_child(barTrack);
  }

  const footer = new St.BoxLayout({ style_class: 'ai-usage-tile-footer' });
  const usageText = new St.Label({
    text: pct > 0
      ? `${pct}% used`
      : (data.meta ?? ''),
    style_class: 'ai-usage-usage-text',
    x_expand: true,
  });
  footer.add_child(usageText);

  const countdown = formatResetCountdown(data.resetAt);
  if (countdown) {
    const resetLabel = new St.Label({
      text: countdown,
      style_class: 'ai-usage-reset-label',
    });
    footer.add_child(resetLabel);
  }
  tile.add_child(footer);

  return tile;
}

export const UsagePopover = GObject.registerClass(
  { GTypeName: 'AiUsagePopover' },
  class UsagePopover extends GObject.Object {
    private _manager: ProviderManagerType;
    private _managerId: number = 0;
    private _tilesBox: St.BoxLayout | null = null;
    private _footerLabel: St.Label | null = null;
    private _footerTimer: number = 0;
    private _refreshButton: St.Button | null = null;

    constructor(manager: ProviderManagerType) {
      super();
      this._manager = manager;

      this._managerId = manager.connect('data-updated', () => {
        this._refresh();
      });
    }

    setRefreshButton(button: St.Button): void {
      this._refreshButton = button;
    }

    buildUI(root: St.BoxLayout): void {
      // Tiles container
      this._tilesBox = new St.BoxLayout({
        style_class: 'ai-usage-tiles',
        vertical: true,
        x_expand: true,
      });
      this._tilesBox.set_name('tiles-box');
      root.add_child(this._tilesBox);

      // Footer
      this._footerLabel = new St.Label({
        text: 'Never updated',
        style_class: 'ai-usage-footer',
      });
      root.add_child(this._footerLabel);

      // Update footer every 30s
      this._footerTimer = GLib.timeout_add_seconds(GLib.PRIORITY_LOW, 30, () => {
        this._updateFooter();
        return GLib.SOURCE_CONTINUE;
      });
    }

    private _setRefreshButtonSensitive(sensitive: boolean): void {
      if (this._refreshButton) {
        this._refreshButton.reactive = sensitive;
        this._refreshButton.style_class = sensitive
          ? 'ai-usage-refresh-btn'
          : 'ai-usage-refresh-btn ai-usage-refresh-btn-busy';
      }
    }

    private _refresh(): void {
      const tilesBox = this._tilesBox;
      if (!tilesBox) return;

      tilesBox.destroy_all_children();

      const data = this._manager.data;
      if (data.length === 0) {
        const empty = new St.Label({
          text: 'No providers enabled \u2014 open Preferences',
          style_class: 'ai-usage-empty',
        });
        tilesBox.add_child(empty);
      } else {
        const sorted = [...data].sort((a, b) => a.name.localeCompare(b.name));
        for (const d of sorted) {
          tilesBox.add_child(buildProviderTile(d));
        }
      }

      this._updateFooter();
    }

    private _updateFooter(): void {
      if (!this._footerLabel) return;
      const { text, stale } = formatLastUpdated(this._manager.lastFetch);
      this._footerLabel.text = text;
      if (stale) {
        this._footerLabel.add_style_class_name('ai-usage-footer-stale');
      } else {
        this._footerLabel.remove_style_class_name('ai-usage-footer-stale');
      }
    }

    destroy(): void {
      if (this._footerTimer) {
        GLib.source_remove(this._footerTimer);
        this._footerTimer = 0;
      }
      if (this._managerId) {
        this._manager.disconnect(this._managerId);
        this._managerId = 0;
      }
      this._tilesBox = null;
    }
  },
);

export type UsagePopoverType = InstanceType<typeof UsagePopover>;
