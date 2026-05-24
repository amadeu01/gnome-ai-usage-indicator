## ADDED Requirements

### Requirement: TOML config file
The application SHALL read configuration from `~/.config/ai-usage-indicator/config.toml`. If the file does not exist, defaults SHALL be used and the file SHALL be created on first run.

#### Scenario: Config file absent on first run
- **WHEN** no config file exists at the path
- **THEN** app starts with defaults; config file is created with default values

#### Scenario: Config file present
- **WHEN** a valid TOML config file exists
- **THEN** all values from the file override defaults

#### Scenario: Config file malformed
- **WHEN** the TOML file contains a syntax error
- **THEN** app logs the error and starts with defaults; does not overwrite the malformed file

### Requirement: Config schema
The config file SHALL support these keys:

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `poll_interval_secs` | integer | `60` | Seconds between provider fetches |
| `enabled_providers` | string array | `["claude-code"]` | Active provider IDs |
| `ollama_host` | string | `"http://localhost:11434"` | Ollama API base URL |
| `anthropic_api_key` | string | `""` | Optional Anthropic Admin API key |

#### Scenario: Read enabled providers
- **WHEN** config has `enabled_providers = ["claude-code", "ollama"]`
- **THEN** both providers are polled each interval

#### Scenario: Default poll interval
- **WHEN** `poll_interval_secs` is absent from config
- **THEN** poll interval is 60 seconds

### Requirement: Config is reloaded on SIGHUP
The application SHALL reload the config file when it receives SIGHUP, applying new values (poll interval, enabled providers) without restarting.

#### Scenario: SIGHUP received
- **WHEN** user sends `kill -HUP <pid>`
- **THEN** config is re-read; poll interval and enabled providers update immediately
