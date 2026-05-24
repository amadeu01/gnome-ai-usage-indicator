## ADDED Requirements

### Requirement: Window anchors below tray icon on open
When the tray icon is activated (clicked), the app window SHALL position itself horizontally centered on the icon and vertically just below it. The window SHALL be clamped to stay fully within the current monitor's work area (no off-screen windows).

#### Scenario: Normal click in top bar
- **WHEN** user clicks the tray icon located at screen position (500, 22)
- **THEN** the window appears with its top edge at approximately y=46 and horizontally centered around x=500

#### Scenario: Icon near right edge of screen
- **WHEN** user clicks the tray icon and the computed window left edge would extend beyond the monitor right edge
- **THEN** the window is shifted left so its right edge aligns with the monitor right edge

#### Scenario: Toggle close
- **WHEN** the window is already visible and user clicks the tray icon again
- **THEN** the window hides (existing toggle behaviour, position not relevant)
