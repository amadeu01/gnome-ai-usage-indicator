import Gio from 'gi://Gio';
import GObject from 'gi://GObject';
import { ProviderData } from './types.js';

const BUS_NAME = 'io.github.amadeu01.AiUsageIndicator';
const OBJECT_PATH = '/io/github/amadeu01/AiUsageIndicator';
const IFACE_NAME = 'io.github.amadeu01.AiUsageIndicator';

const IFACE_XML = `
<node>
  <interface name="${IFACE_NAME}">
    <method name="GetProviderData">
      <arg type="s" direction="out" name="json"/>
    </method>
    <method name="Refresh">
    </method>
    <method name="GetConfig">
      <arg type="s" direction="out" name="json"/>
    </method>
    <signal name="DataUpdated">
      <arg type="s" name="json"/>
    </signal>
  </interface>
</node>`;

function rustDataToProviderData(item: any): ProviderData {
  return {
    id: item.id ?? 'unknown',
    name: item.name ?? 'Unknown',
    utilization: item.utilization ?? 0,
    usedTokens: Math.round(item.utilization ?? 0),
    limitTokens: 100,
    usedCredits: item.usedCredits ?? null,
    limitCredits: item.limitCredits ?? null,
    resetAt: item.resetAt ? new Date(item.resetAt) : null,
    error: item.error ?? null,
    windowLabel: item.windowLabel ?? null,
    paceInfo: item.paceInfo ?? null,
    meta: item.meta ?? null,
  };
}
function offlineProviders(): ProviderData[] {
  return [
    {
      id: 'daemon-offline',
      name: 'AI Usage Daemon',
      utilization: 0,
      usedTokens: 0,
      limitTokens: 100,
      usedCredits: null,
      limitCredits: null,
      resetAt: null,
      error: 'Daemon offline',
      windowLabel: null,
      paceInfo: null,
      meta: null,
    },
  ];
}

export const ProviderManager = GObject.registerClass(
  {
    GTypeName: 'AiUsageProviderManager',
    Signals: {
      'data-updated': {},
    },
  },
  class ProviderManager extends GObject.Object {
    private _proxy: Gio.DBusProxy | null = null;
    private _signalId: number = 0;
    private _data: ProviderData[] = [];
    private _fetching: boolean = false;
    private _lastFetch: Date | null = null;

    constructor() {
      super();
      this._initProxy();
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

    private _initProxy(): void {
      try {
        const ifaceInfo = Gio.DBusNodeInfo.new_for_xml(IFACE_XML).interfaces[0];
        this._proxy = new Gio.DBusProxy({
          g_connection: Gio.DBus.session,
          g_name: BUS_NAME,
          g_object_path: OBJECT_PATH,
          g_interface_name: IFACE_NAME,
          g_interface_info: ifaceInfo,
          g_flags: Gio.DBusProxyFlags.DO_NOT_AUTO_START,
        });

        this._proxy.init(null);

        this._signalId = this._proxy.connect('g-signal', (_proxy, _sender, signalName, params) => {
          if (signalName === 'DataUpdated') {
            try {
              const json = params.get_child_value(0).get_string()[0];
              const parsed = JSON.parse(json);
              this._data = parsed.map(rustDataToProviderData);
              this._lastFetch = new Date();
              this.emit('data-updated');
            } catch {
              // ignore malformed signal
            }
          }
        });
      } catch {
        this._proxy = null;
      }
    }

    async fetchAll(): Promise<void> {
      if (this._fetching) return;
      this._fetching = true;

      try {
        if (!this._proxy) {
          this._initProxy();
        }

        if (!this._proxy) {
          this._data = offlineProviders();
          this._lastFetch = new Date();
          this.emit('data-updated');
          return;
        }

        const result = await new Promise<string | null>((resolve) => {
          try {
            this._proxy!.call(
              'GetProviderData',
              null,
              Gio.DBusCallFlags.NONE,
              5000,
              null,
              (_proxy: Gio.DBusProxy, asyncResult: Gio.AsyncResult) => {
                try {
                  const reply = this._proxy!.call_finish(asyncResult);
                  const json = reply.get_child_value(0).get_string()[0];
                  resolve(json);
                } catch {
                  resolve(null);
                }
              }
            );
          } catch {
            resolve(null);
          }
        });

        if (result === null) {
          this._data = offlineProviders();
          this._proxy = null;
        } else {
          const parsed = JSON.parse(result);
          this._data = parsed.map(rustDataToProviderData);
        }

        this._lastFetch = new Date();
        this.emit('data-updated');
      } finally {
        this._fetching = false;
      }
    }

    destroy(): void {
      if (this._proxy && this._signalId) {
        this._proxy.disconnect(this._signalId);
        this._signalId = 0;
      }
      this._proxy = null;
    }
  }
);

export type ProviderManagerType = InstanceType<typeof ProviderManager>;
