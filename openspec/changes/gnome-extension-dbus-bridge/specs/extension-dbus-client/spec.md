## ADDED Requirements

### Requirement: ProviderManager uses DBus proxy instead of HTTP
The GNOME Extension `ProviderManager` SHALL use a `Gio.DBusProxy` to call `GetProviderData()` on `io.github.amadeu01.AiUsageIndicator` instead of making direct HTTP requests to AI provider APIs.

#### Scenario: Daemon running, data available
- **WHEN** the extension calls `fetchAll()` and the daemon is running
- **THEN** `ProviderManager` calls `GetProviderData()` via DBus
- **AND** parses the returned JSON into the existing `ProviderData[]` TypeScript type
- **AND** emits `data-updated` signal

#### Scenario: Daemon not running
- **WHEN** the extension calls `fetchAll()` and the daemon DBus name is not present
- **THEN** `ProviderManager` sets an error state on all providers with message "Daemon offline"
- **AND** emits `data-updated` signal so the UI reflects the offline state

### Requirement: Extension subscribes to DataUpdated signal for push updates
The GNOME Extension SHALL subscribe to the `DataUpdated` DBus signal from the daemon. When the signal arrives, the extension SHALL update its displayed data without waiting for the next poll timer tick.

#### Scenario: Daemon emits DataUpdated
- **WHEN** the daemon emits a `DataUpdated` signal
- **THEN** the extension receives it within one GLib event loop iteration
- **AND** updates `ProviderManager.data` and emits `data-updated`

### Requirement: Extension provider HTTP code removed
The GNOME Extension SHALL NOT contain any HTTP fetch logic for AI providers. The files `ts-src/src/providers/anthropicApi.ts`, `ts-src/src/providers/claudeCode.ts`, and `ts-src/src/providers/codex.ts` SHALL be deleted. The `ProviderManager` SHALL NOT import `Soup` or make any HTTP calls.

#### Scenario: Build check
- **WHEN** the extension TypeScript is compiled
- **THEN** no imports of Soup or provider-specific HTTP modules exist in `providerManager.ts`

### Requirement: anthropic-api-key removed from GSettings
The GSettings schema SHALL NOT contain the `anthropic-api-key` key. API keys are stored in the daemon config file only.

#### Scenario: Schema valid without api-key
- **WHEN** the extension schema is compiled with `glib-compile-schemas`
- **THEN** no `anthropic-api-key` key exists in the compiled schema

### Requirement: Extension prefs page updated
The extension preferences page (`prefs.ts`) SHALL remove the API key input field and instead display a notice that API keys are configured in the daemon config file at `~/.config/ai-usage-indicator/config.toml`.

#### Scenario: Prefs opened
- **WHEN** the user opens extension preferences
- **THEN** no API key input field is shown
- **AND** a label or link directs the user to the config file location

### Requirement: Extension reconnects when daemon restarts
If the DBus proxy loses the service name (daemon stopped), the extension SHALL detect this and attempt to reconnect on the next `fetchAll()` call rather than crashing.

#### Scenario: Daemon restarted
- **WHEN** the daemon process restarts while the extension is enabled
- **THEN** within one poll cycle the extension successfully reconnects and resumes showing data
