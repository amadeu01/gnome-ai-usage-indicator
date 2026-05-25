// GJS provides TextDecoder globally
declare class TextDecoder {
  constructor(encoding?: string);
  decode(input?: Uint8Array | ArrayBuffer): string;
}

// Ambient type declarations for GJS / GNOME Shell modules.
// These modules are provided by the GJS runtime; the declarations here
// give TypeScript enough info to compile without @girs packages installed.

declare module 'gi://GLib' {
  namespace GLib {
    const PRIORITY_DEFAULT: number;
    const PRIORITY_LOW: number;
    function timeout_add_seconds(priority: number, interval: number, func: () => boolean): number;
    function timeout_add(priority: number, interval: number, func: () => boolean): number;
    function source_remove(tag: number): boolean;
    function idle_add(priority: number, func: () => boolean): number;
    function get_home_dir(): string;
    function build_filename(...components: string[]): string;
    function markup_escape_text(text: string, length: number): string;
    const SOURCE_REMOVE: boolean;
    const SOURCE_CONTINUE: boolean;
  }
  export default GLib;
}

declare module 'gi://Gio' {
  namespace Gio {
    interface Icon {
      // GJS Gio.Icon interface
    }
    function icon_new_for_string(str: string): Icon;
    class File {
      static new_for_path(path: string): File;
      get_path(): string | null;
      query_exists(cancellable?: any): boolean;
      load_contents(cancellable?: any): [boolean, Uint8Array, string];
      enumerate_children(attributes: string, flags: any, cancellable?: any): FileEnumerator;
    }
    class FileEnumerator {
      next_file(cancellable?: any): FileInfo | null;
      close(cancellable?: any): void;
    }
    class FileInfo {
      get_name(): string;
    }
    class Settings {
      constructor(schemaId: string);
      get_strv(key: string): string[];
      set_strv(key: string, value: string[]): void;
      get_int(key: string): number;
      set_int(key: string, value: number): void;
      get_string(key: string): string;
      set_string(key: string, value: string): void;
      connect(signal: string, callback: (...args: any[]) => any): number;
      disconnect(id: number): void;
    }
    namespace FileQueryInfoFlags {
      const NONE: number;
    }
    namespace FileMonitorFlags {
      const NONE: number;
    }
    class DBusNodeInfo {
      static new_for_xml(xml: string): DBusNodeInfo;
      interfaces: DBusInterfaceInfo[];
    }
    class DBusInterfaceInfo {
      // GJS DBus interface info
    }
    enum DBusProxyFlags {
      NONE = 0,
      DO_NOT_AUTO_START = 4,
    }
    enum DBusCallFlags {
      NONE = 0,
    }
    class DBusProxy {
      constructor(params: any);
      init(cancellable: any): void;
      connect(signal: string, callback: (...args: any[]) => any): number;
      disconnect(id: number): void;
      call(
        methodName: string,
        parameters: any,
        flags: DBusCallFlags,
        timeoutMsec: number,
        cancellable: any,
        callback: (proxy: DBusProxy, result: AsyncResult) => void
      ): void;
      call_finish(result: AsyncResult): GLib.Variant;
    }
    class AsyncResult {
      // GJS async result type
    }
    namespace GLib {
      class Variant {
        get_child_value(index: number): Variant;
        get_string(): [string, number];
      }
    }
    const DBus: {
      session: any;
    };
  }
  export default Gio;
}

declare module 'gi://GObject' {
  namespace GObject {
    class Object {
      connect(signal: string, callback: (...args: any[]) => any): number;
      disconnect(id: number): void;
      emit(signal: string, ...args: any[]): void;
      block_signal_handler(id: number): void;
      unblock_signal_handler(id: number): void;
    }
    function registerClass(meta: any, klass: any): any;
    function registerClass(klass: any): any;
    const TYPE_NONE: any;
    const TYPE_STRING: any;
    const TYPE_INT: any;
    const TYPE_BOOLEAN: any;
    const ParamSpec: any;
  }
  export default GObject;
}

declare module 'gi://St' {
  namespace St {
    class Widget {
      constructor(params?: any);
      style_class: string;
      style: string;
      visible: boolean;
      reactive: boolean;
      x_expand: boolean;
      add_child(child: Widget): void;
      remove_child(child: Widget): void;
      destroy(): void;
      add_style_class_name(name: string): void;
      remove_style_class_name(name: string): void;
      get_children(): Widget[];
      get_n_children(): number;
      destroy_all_children(): void;
      set_name(name: string): void;
      get_name(): string;
    }
    class BoxLayout extends Widget {
      constructor(params?: any);
      vertical: boolean;
    }
    class Label extends Widget {
      constructor(params?: any);
      text: string;
    }
    class Icon extends Widget {
      constructor(params?: any);
      icon_name: string;
      icon_size: number;
    }
    class Button extends Widget {
      constructor(params?: any);
      reactive: boolean;
      connect(signal: string, callback: (...args: any[]) => any): number;
    }
    class Bin extends Widget {
      constructor(params?: any);
      child: Widget | null;
    }
    namespace Align {
      const START: number;
      const MIDDLE: number;
      const END: number;
    }
  }
  export default St;
}


declare module 'gi://Adw' {
  namespace Adw {
    class PreferencesWindow {
      constructor(params?: any);
      add(page: PreferencesPage): void;
      present(): void;
    }
    class PreferencesPage {
      constructor(params?: any);
      title: string;
      icon_name: string;
      add(group: PreferencesGroup): void;
    }
    class PreferencesGroup {
      constructor(params?: any);
      title: string;
      add(row: any): void;
    }
    class SwitchRow {
      constructor(params?: any);
      title: string;
      active: boolean;
      connect(signal: string, callback: (...args: any[]) => any): number;
    }
    class EntryRow {
      constructor(params?: any);
      title: string;
      text: string;
      connect(signal: string, callback: (...args: any[]) => any): number;
    }
    class PasswordEntryRow {
      constructor(params?: any);
      title: string;
      text: string;
      connect(signal: string, callback: (...args: any[]) => any): number;
    }
    class SpinRow {
      constructor(params?: any);
      title: string;
      value: number;
      adjustment: any;
      connect(signal: string, callback: (...args: any[]) => any): number;
    }
    class Banner {
      constructor(params?: any);
      title: string;
      revealed: boolean;
    }
    class ActionRow {
      constructor(params?: any);
      title: string;
      subtitle: string;
      activatable: boolean;
      add_suffix(widget: any): void;
    }
  }
  export default Adw;
}

declare module 'gi://Gtk' {
  namespace Gtk {
    class Adjustment {
      constructor(params?: any);
      value: number;
      lower: number;
      upper: number;
      step_increment: number;
    }
    class Image {
      constructor(params?: any);
      icon_name: string;
    }
  }
  export default Gtk;
}

declare module 'resource:///org/gnome/shell/ui/main.js' {
  const panel: {
    addToStatusArea(role: string, indicator: any, position?: number, box?: string): any;
    _rightBox: any;
  };
  const screenShield: {
    connect(signal: string, callback: (...args: any[]) => void): number;
    disconnect(id: number): void;
    active: boolean;
  };
  export { panel, screenShield };
}

declare module 'resource:///org/gnome/shell/ui/panelMenu.js' {
  class Button {
    constructor(menuAlignment: number, nameText: string, dontCreateMenu?: boolean);
    menu: any;
    add_child(child: any): void;
    connect(signal: string, callback: (...args: any[]) => any): number;
    disconnect(id: number): void;
    destroy(): void;
  }
  export { Button };
}

declare module 'resource:///org/gnome/shell/ui/popupMenu.js' {
  class PopupMenuSection {
    constructor();
    actor: any;
    box: any;
    addMenuItem(item: any): void;
    removeAll(): void;
  }
  class PopupSeparatorMenuItem {
    constructor(text?: string);
  }
  class PopupBaseMenuItem {
    constructor(params?: any);
    actor: any;
  }
  export { PopupMenuSection, PopupSeparatorMenuItem, PopupBaseMenuItem };
}

declare module 'resource:///org/gnome/shell/extensions/extension.js' {
  class Extension {
    getSettings(schemaId?: string): any;
    path: string;
    uuid: string;
    metadata: any;
    enable(): void;
    disable(): void;
  }
  export { Extension };
}

declare module 'resource:///org/gnome/Shell/Extensions/js/extensions/prefs.js' {
  class ExtensionPreferences {
    getSettings(schemaId?: string): any;
    path: string;
    uuid: string;
    metadata: any;
    fillPreferencesWindow(window: any): void;
  }
  export { ExtensionPreferences };
}
