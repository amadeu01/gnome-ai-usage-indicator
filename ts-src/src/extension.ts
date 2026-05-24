import GLib from 'gi://GLib';
import { Extension } from 'resource:///org/gnome/shell/extensions/extension.js';
import * as Main from 'resource:///org/gnome/shell/ui/main.js';
import { ProviderManager } from './providerManager.js';
import { PanelIndicator } from './panelIndicator.js';

export default class AiUsageExtension extends Extension {
  private _indicator: any = null;
  private _manager: any = null;
  private _pollTimer: number = 0;
  private _screenShieldId: number = 0;
  private _settingsId: number = 0;
  private _settings: any = null;

  override enable(): void {
    this._settings = this.getSettings();
    this._manager = new (ProviderManager as any)(this._settings);
    this._indicator = new (PanelIndicator as any)(this._manager, this.path);

    Main.panel.addToStatusArea(this.uuid, this._indicator);

    this._startPolling();

    if (Main.screenShield) {
      this._screenShieldId = Main.screenShield.connect(
        'notify::active',
        () => this._onScreenShieldChanged()
      );
    }

    this._settingsId = this._settings.connect(
      'changed::poll-interval',
      () => this._restartPolling()
    );

    // Initial fetch
    this._manager.fetchAll().catch(() => {});
  }

  override disable(): void {
    if (this._screenShieldId && Main.screenShield) {
      Main.screenShield.disconnect(this._screenShieldId);
      this._screenShieldId = 0;
    }

    if (this._settingsId && this._settings) {
      this._settings.disconnect(this._settingsId);
      this._settingsId = 0;
    }

    this._stopPolling();

    if (this._indicator) {
      this._indicator.destroy();
      this._indicator = null;
    }

    this._manager = null;
    this._settings = null;
  }

  private _startPolling(): void {
    this._stopPolling();
    const interval = this._settings.get_int('poll-interval');
    this._pollTimer = GLib.timeout_add_seconds(GLib.PRIORITY_LOW, interval, () => {
      if (!Main.screenShield?.active) {
        this._manager?.fetchAll().catch(() => {});
      }
      return GLib.SOURCE_CONTINUE;
    });
  }

  private _stopPolling(): void {
    if (this._pollTimer) {
      GLib.source_remove(this._pollTimer);
      this._pollTimer = 0;
    }
  }

  private _restartPolling(): void {
    this._startPolling();
  }

  private _onScreenShieldChanged(): void {
    if (Main.screenShield?.active) {
      this._stopPolling();
    } else {
      this._manager?.fetchAll().catch(() => {});
      this._startPolling();
    }
  }
}
