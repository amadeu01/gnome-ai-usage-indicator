## ADDED Requirements

### Requirement: Ollama running-models tile
When the Ollama provider is enabled, the extension SHALL GET `{ollamaHost}/api/ps` (default `http://localhost:11434`) and display the result as a provider tile.

#### Scenario: Ollama running with loaded models
- **WHEN** the API responds with one or more models in the `models` array
- **THEN** tile shows provider name "Ollama", lists each loaded model name, and shows total VRAM used

#### Scenario: Ollama running but no models loaded
- **WHEN** the API responds with an empty `models` array
- **THEN** tile shows "Ollama" with message "No models loaded"

#### Scenario: Ollama not running
- **WHEN** connection to `ollamaHost/api/ps` is refused or times out (3s)
- **THEN** tile shows "Ollama" with `error: "Not running"`

### Requirement: Ollama has no usage limits
The Ollama tile SHALL NOT show a progress bar or reset countdown. `usedTokens`, `limitTokens`, `usedCredits`, `limitCredits`, and `resetAt` SHALL all be null or 0.

#### Scenario: Tile renders without usage bar
- **WHEN** Ollama tile is rendered in the popover
- **THEN** no progress bar is shown; only model names and VRAM are displayed

### Requirement: Configurable Ollama host
The Ollama provider SHALL read the host URL from the `ollama-host` GSettings key (default `http://localhost:11434`).

#### Scenario: Custom host configured
- **WHEN** user sets `ollama-host` to `http://192.168.1.5:11434`
- **THEN** all requests go to that host instead of localhost
