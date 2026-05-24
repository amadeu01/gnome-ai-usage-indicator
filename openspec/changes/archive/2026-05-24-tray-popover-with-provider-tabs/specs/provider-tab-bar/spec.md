## ADDED Requirements

### Requirement: Tab bar groups providers
The window SHALL display a tab bar at the top containing one tab per unique provider `name`. Each tab shows the provider name as its label. The content area below the tab bar SHALL show only the selected provider's data.

#### Scenario: Single provider
- **WHEN** only "Claude Code" entries are in the data
- **THEN** one tab labelled "Claude Code" is shown and its content is active

#### Scenario: Multiple providers
- **WHEN** data contains entries for "Claude Code" and "Ollama"
- **THEN** two tabs are shown; clicking each switches the content area to that provider's data

#### Scenario: Tab order stability
- **WHEN** data refreshes and providers are returned in different order
- **THEN** tab order remains alphabetical and the previously selected tab stays selected (if provider still exists)

### Requirement: Empty and error states per tab
Each provider tab SHALL display an error message within its own content area when the provider returned an error, rather than hiding the tab.

#### Scenario: Provider fetch error
- **WHEN** a provider returns an entry with `error` set
- **THEN** the tab for that provider is still shown and displays the error string inside the content area
