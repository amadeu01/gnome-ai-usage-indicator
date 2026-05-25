## REMOVED Requirements

### Requirement: ProviderId type export
**Reason**: `ProviderId` type in `types.ts:25` has zero imports across the entire TypeScript codebase. Dead code.
**Migration**: None required — no consumers exist.

#### Scenario: types.ts no longer exports ProviderId
- **WHEN** `types.ts` is compiled
- **THEN** it SHALL NOT contain the `ProviderId` type alias or the line `export type ProviderId = ...`

### Requirement: Soup ambient declarations
**Reason**: The `Soup` module declaration in `ambient.d.ts:200-223` is not imported by any TypeScript source file. Dead type declaration.
**Migration**: Re-add from GJS documentation if Soup is needed in the future.

#### Scenario: ambient.d.ts has no Soup module
- **WHEN** `ambient.d.ts` is read
- **THEN** it SHALL NOT contain `declare module 'gi://Soup'`

### Requirement: Clutter ambient declarations
**Reason**: The `Clutter` module declaration in `ambient.d.ts:181-198` is not imported by any TypeScript source file. Dead type declaration.
**Migration**: Re-add from GJS documentation if Clutter is needed in the future.

#### Scenario: ambient.d.ts has no Clutter module
- **WHEN** `ambient.d.ts` is read
- **THEN** it SHALL NOT contain `declare module 'gi://Clutter'`

## ADDED Requirements

### Requirement: Inline _restartPolling
The `_restartPolling()` method in `extension.ts` SHALL be removed. Its single call site SHALL call `this._startPolling()` directly.

#### Scenario: Settings change triggers polling restart
- **WHEN** the `poll-interval` setting changes
- **THEN** `_startPolling()` SHALL be called directly in the settings change handler

### Requirement: Store tilesBox reference
`UsagePopover` SHALL store the `tilesBox` widget as a private field at construction time. The `_findTilesBox()` method SHALL be removed.

#### Scenario: Refresh uses stored reference
- **WHEN** `_refresh()` is called on `UsagePopover`
- **THEN** it SHALL use `this._tilesBox` directly instead of walking the widget tree

#### Scenario: tilesBox is cleaned up on destroy
- **WHEN** `UsagePopover.destroy()` is called
- **THEN** the `_tilesBox` reference SHALL be nulled
