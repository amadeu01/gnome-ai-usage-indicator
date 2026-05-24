## ADDED Requirements

### Requirement: GTK4 popover window
The application SHALL display a `gtk::Window` (undecorated, popup-menu hint) when the tray icon is activated. The window SHALL close when focus is lost.

#### Scenario: Tray activated
- **WHEN** user left-clicks the tray icon
- **THEN** an undecorated window appears near the tray with an Adwaita-styled card

#### Scenario: Click outside popover
- **WHEN** popover is visible and user clicks outside the window
- **THEN** popover closes

### Requirement: Per-provider section groups
The popover SHALL render one section group per enabled provider. Each group SHALL show the provider name as a bold header. Multiple usage windows within the same provider (Session, Weekly) SHALL be stacked as labelled subsections inside the group.

#### Scenario: Claude Code enabled
- **WHEN** popover opens with Claude Code enabled
- **THEN** a "Claude" group appears with "Session" and "Weekly" subsections, each with a progress bar, percentage text, and reset countdown

#### Scenario: Ollama enabled and running
- **WHEN** popover opens with Ollama enabled and models loaded
- **THEN** an "Ollama" group appears listing loaded model names

#### Scenario: No providers enabled
- **WHEN** all providers are disabled in config
- **THEN** popover shows "No providers configured — edit ~/.config/ai-usage-indicator/config.toml"

### Requirement: Progress bar with color thresholds
Each usage subsection SHALL show a horizontal `gtk::ProgressBar` styled with CSS. Color class SHALL change at 80% (warning, amber) and 95% (critical, red).

#### Scenario: Below 80%
- **WHEN** usage is 22%
- **THEN** bar is 22% wide, green CSS class applied

#### Scenario: At or above 80%
- **WHEN** usage is 85%
- **THEN** bar is 85% wide, amber CSS class applied

### Requirement: Pace indicator on weekly window
The weekly usage subsection SHALL show a pace line: compares consumed fraction to elapsed fraction of the 7-day window.

#### Scenario: Ahead of pace
- **WHEN** 30% of the week has elapsed but 50% of weekly tokens are consumed
- **THEN** pace line shows "Pace: Ahead (+67%)" in amber

#### Scenario: Behind pace
- **WHEN** 50% of the week has elapsed but 20% of tokens consumed
- **THEN** pace line shows "Pace: Behind (-60%)" in muted text

### Requirement: Manual refresh button
The popover header SHALL include a refresh button. Clicking it fetches all providers immediately and updates the UI.

#### Scenario: Click refresh
- **WHEN** user clicks the refresh button
- **THEN** all providers are fetched; button is insensitive during fetch; UI updates on completion

### Requirement: Last-updated footer
The popover SHALL show a footer label with the time since the last successful fetch.

#### Scenario: Recent fetch
- **WHEN** data was fetched under 60 seconds ago
- **THEN** footer shows "Updated just now"

#### Scenario: Stale data
- **WHEN** last fetch was more than 5 minutes ago
- **THEN** footer label uses a warning CSS class
