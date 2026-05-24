## ADDED Requirements

### Requirement: Systemd user service unit
The project SHALL ship a `ai-usage-indicator.service` systemd user unit file. `make install` SHALL copy it to `~/.config/systemd/user/` and enable it.

#### Scenario: make install runs
- **WHEN** user runs `make install`
- **THEN** binary is copied to `~/.local/bin/ai-usage-indicator` and service file is installed to `~/.config/systemd/user/`; `systemctl --user enable --now ai-usage-indicator` is run

#### Scenario: Service starts on login
- **WHEN** user logs into a graphical session
- **THEN** systemd user instance starts `ai-usage-indicator.service` and tray icon appears

### Requirement: make targets
The Makefile SHALL provide `build`, `install`, `run`, `uninstall`, and `clean` targets using `cargo`.

#### Scenario: make build
- **WHEN** user runs `make build`
- **THEN** `cargo build --release` runs; binary produced at `target/release/ai-usage-indicator`

#### Scenario: make uninstall
- **WHEN** user runs `make uninstall`
- **THEN** systemd service is stopped and disabled; binary and service file are removed; config file is NOT removed
