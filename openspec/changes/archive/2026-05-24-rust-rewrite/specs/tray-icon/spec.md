## ADDED Requirements

### Requirement: StatusNotifierItem tray icon
The application SHALL register a StatusNotifierItem D-Bus service via `ksni`. The icon SHALL use the symbolic SVG from `res/icons/ai-usage-indicator-symbolic.svg` as the icon name.

#### Scenario: App starts
- **WHEN** the binary launches
- **THEN** a tray icon appears in the system status area within 1 second

#### Scenario: AppIndicator extension not present on GNOME
- **WHEN** GNOME Shell has no StatusNotifierItem support
- **THEN** the app logs a warning to stderr and continues running (tray hidden, popover still accessible via CLI flag)

### Requirement: Left-click toggles popover
The tray icon SHALL toggle the popover window on left-click activation.

#### Scenario: Popover closed, user left-clicks tray
- **WHEN** popover is not visible and user activates the tray icon
- **THEN** popover window appears near the tray icon

#### Scenario: Popover open, user left-clicks tray
- **WHEN** popover is visible and user activates the tray icon
- **THEN** popover window closes

### Requirement: Tray icon reflects max usage level
The tray icon tooltip SHALL show the highest usage percentage across all enabled providers. Icon SHALL change color class at warning (≥80%) and critical (≥95%) thresholds.

#### Scenario: All providers below 80%
- **WHEN** all providers report less than 80% utilization
- **THEN** tooltip shows "AI Usage: X% max" with no warning indicator

#### Scenario: Any provider at or above 95%
- **WHEN** any provider reports ≥95% utilization
- **THEN** tooltip shows "AI Usage: X% — Critical" and icon uses critical color
