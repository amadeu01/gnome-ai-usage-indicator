## ADDED Requirements

### Requirement: Fetch running models from Ollama REST API
The Ollama provider SHALL GET `{ollama_host}/api/ps` using `reqwest` with a 3-second timeout. It SHALL return a `ProviderData` with model information.

#### Scenario: Models loaded
- **WHEN** Ollama responds with `{ "models": [{ "name": "llama3", "size_vram": 4000000000 }] }`
- **THEN** entry has `meta: Some("llama3 (3.7 GB)")`, `error: None`

#### Scenario: Ollama running, no models loaded
- **WHEN** Ollama responds with `{ "models": [] }`
- **THEN** entry has `meta: Some("No models loaded")`, `error: None`

#### Scenario: Ollama not running
- **WHEN** connection to `ollama_host` is refused or times out
- **THEN** entry has `error: Some("Not running")`, `meta: None`

### Requirement: Ollama has no usage limits
The Ollama `ProviderData` entry SHALL have `utilization: 0.0`, `limit: 0`, `reset_at: None`, `used_credits: None`, `limit_credits: None`. No progress bar SHALL be shown for this entry.

#### Scenario: Ollama tile rendered
- **WHEN** popover renders the Ollama group
- **THEN** no progress bar widget is created; only model names (or error) are shown

### Requirement: Configurable Ollama host
The Ollama provider SHALL read the host from `config.ollama_host` (default `http://localhost:11434`).

#### Scenario: Custom host
- **WHEN** config has `ollama_host = "http://192.168.1.5:11434"`
- **THEN** all requests use that host
