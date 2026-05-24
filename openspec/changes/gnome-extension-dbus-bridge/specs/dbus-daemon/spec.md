## ADDED Requirements

### Requirement: Daemon exposes DBus service on session bus
The Rust daemon SHALL claim the well-known name `io.github.amadeu01.AiUsageIndicator` on the session DBus and expose an object at `/io/github/amadeu01/AiUsageIndicator` implementing the interface `io.github.amadeu01.AiUsageIndicator`.

#### Scenario: Service is available after daemon starts
- **WHEN** the daemon process starts successfully
- **THEN** `busctl --user list` includes `io.github.amadeu01.AiUsageIndicator`

#### Scenario: Service is gone after daemon stops
- **WHEN** the daemon process exits
- **THEN** `busctl --user list` no longer includes `io.github.amadeu01.AiUsageIndicator`

### Requirement: GetProviderData method returns current provider data
The daemon SHALL expose a `GetProviderData` DBus method that returns a JSON-serialized array of provider data objects, each containing at minimum: `id`, `name`, `utilization`, `error` (nullable), `usedCredits` (nullable), `limitCredits` (nullable), `resetAt` (nullable ISO8601 string).

#### Scenario: Data available
- **WHEN** a DBus client calls `GetProviderData()`
- **THEN** the method returns a JSON string parseable as an array
- **AND** each element has an `id` field matching a known provider ID

#### Scenario: Fetch not yet complete
- **WHEN** `GetProviderData()` is called before the first fetch cycle completes
- **THEN** the method returns an empty JSON array `[]`

### Requirement: Refresh method triggers immediate re-fetch
The daemon SHALL expose a `Refresh` DBus method that cancels the current sleep interval and immediately re-fetches all enabled providers.

#### Scenario: Refresh called
- **WHEN** a DBus client calls `Refresh()`
- **THEN** the daemon begins fetching all providers within 100ms
- **AND** emits `DataUpdated` when the fetch completes

### Requirement: DataUpdated signal emitted after each fetch
The daemon SHALL emit a `DataUpdated` DBus signal after each completed fetch cycle. The signal carries one string argument: the same JSON array returned by `GetProviderData`.

#### Scenario: Periodic fetch completes
- **WHEN** the daemon's background fetch timer fires and all providers respond
- **THEN** a `DataUpdated` signal is emitted on the session bus

#### Scenario: Refresh-triggered fetch completes
- **WHEN** a `Refresh()` call triggers a fetch and it completes
- **THEN** a `DataUpdated` signal is emitted

### Requirement: Background fetch loop runs at configured interval
The daemon SHALL fetch all enabled providers on a repeating interval (default 60 seconds) configurable via the config file (`poll_interval_secs`).

#### Scenario: Default interval
- **WHEN** no `poll_interval_secs` is set in config
- **THEN** the daemon fetches every 60 seconds

#### Scenario: Custom interval
- **WHEN** `poll_interval_secs = 30` is set in config
- **THEN** the daemon fetches every 30 seconds

### Requirement: GetConfig method returns current daemon config
The daemon SHALL expose a `GetConfig` DBus method returning a JSON-serialized object of the current config (excluding sensitive values like API keys, which are represented as boolean `present: true/false`).

#### Scenario: Config readable
- **WHEN** a DBus client calls `GetConfig()`
- **THEN** the method returns a JSON object with `enabledProviders` array and `pollIntervalSecs` integer

### Requirement: Daemon runs without display server
The daemon SHALL start and operate without `DISPLAY` or `WAYLAND_DISPLAY` environment variables. It MUST NOT depend on GTK, GDK, or any display toolkit.

#### Scenario: No display env set
- **WHEN** daemon is started with no display environment variables
- **THEN** daemon starts successfully and claims DBus name

### Requirement: Config file path is stable
The daemon SHALL read and write config from `~/.config/ai-usage-indicator/config.toml`. It SHALL create the directory and a default config if neither exists.

#### Scenario: First run
- **WHEN** no config file exists
- **THEN** daemon creates `~/.config/ai-usage-indicator/config.toml` with defaults and starts normally
