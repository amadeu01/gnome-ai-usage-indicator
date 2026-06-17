# ────────────────────────────────────────────────────────────────
# ai-usage-indicator — unified build system
#
# DAEMON (Rust / cargo):
#   Headless tokio process that fetches AI provider data and
#   exposes it over DBus at io.github.amadeu01.AiUsageIndicator.
#
# EXTENSION (TypeScript / pnpm):
#   GNOME Shell extension that reads from the daemon via DBus
#   and renders provider usage in the top panel.
#
# Typical workflow:
#   make install        — build + install both, start daemon service
#   make run            — run daemon in foreground (dev)
#   make refresh        — trigger immediate daemon data refresh via DBus
#   make debug          — run daemon with RUST_LOG=debug (dev)
#   make ext-reload     — reinstall + restart extension (dev)
#   make stop           — stop daemon
#   make uninstall      — remove daemon + extension
#   make clean          — delete all build artifacts
# ────────────────────────────────────────────────────────────────

BINARY      = ai-usage-indicator
INSTALL_BIN = $(HOME)/.local/bin/$(BINARY)
SERVICE_DIR = $(HOME)/.config/systemd/user
SERVICE_FILE = $(BINARY).service
PNPM        = npm

EXT_UUID    = gnome-ai-usage-indicator@amadeu01.github.io
EXT_SRC     = ts-src
EXT_DIST    = $(EXT_SRC)/dist
EXT_INSTALL = $(HOME)/.local/share/gnome-shell/extensions/$(EXT_UUID)

.PHONY: build build-daemon build-ext \
        install install-daemon install-ext \
        run debug refresh status stop \
        uninstall uninstall-daemon uninstall-ext \
        ext-reload typecheck \
        clean clean-daemon clean-ext \
        help

# ── Build ────────────────────────────────────────────────────────

## build         — compile both daemon (release) and extension
build: build-daemon build-ext

## build-daemon  — compile Rust daemon (release binary)
build-daemon:
	cargo +nightly build --release

## build-ext     — typecheck + compile TypeScript extension to dist/
build-ext:
	cd $(EXT_SRC) && $(PNPM) run build

## typecheck     — type-check extension without emitting files
typecheck:
	cd $(EXT_SRC) && $(PNPM) run typecheck

# ── Install ──────────────────────────────────────────────────────

## install       — build + install daemon and extension; start service
install: install-daemon install-ext
	@echo ""
	@echo "Install complete. To activate the extension:"
	@echo "  1. Log out and log back in  (Wayland — required on first install)"
	@echo "  2. Open GNOME Extensions and enable '$(EXT_UUID)'"
	@echo "  OR run: gnome-extensions enable $(EXT_UUID)"

## install-daemon — install binary + systemd service; enable + start
install-daemon: build-daemon
	install -Dm755 target/release/$(BINARY) $(INSTALL_BIN)
	install -Dm644 $(SERVICE_FILE) $(SERVICE_DIR)/$(SERVICE_FILE)
	systemctl --user daemon-reload
	systemctl --user enable --now $(SERVICE_FILE)
	@echo "Daemon installed and started"

## install-ext   — build + copy extension files to GNOME extension dir
install-ext: build-ext
	mkdir -p $(EXT_INSTALL)
	cp -r $(EXT_DIST)/* $(EXT_INSTALL)/
	cp $(EXT_SRC)/metadata.json $(EXT_INSTALL)/
	cp $(EXT_SRC)/stylesheet.css $(EXT_INSTALL)/
	cp -r $(EXT_SRC)/schemas $(EXT_INSTALL)/
	cp -r $(EXT_SRC)/icons $(EXT_INSTALL)/
	glib-compile-schemas $(EXT_INSTALL)/schemas/
	@echo "Extension installed to $(EXT_INSTALL)"

# ── Dev ──────────────────────────────────────────────────────────

## run           — stop existing instance; run daemon in foreground (debug build)
run: stop
	cargo run

## debug         — run daemon in foreground with full debug logging
debug: stop
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run

## ext-reload    — reinstall extension and restart GNOME Shell (X11 only)
ext-reload: install-ext
	@if [ -n "$$DISPLAY" ]; then \
		gnome-extensions disable $(EXT_UUID) 2>/dev/null; \
		gnome-extensions enable $(EXT_UUID) 2>/dev/null; \
		echo "Extension reloaded"; \
	else \
		echo "Wayland: log out and back in to reload extension"; \
	fi

## refresh       — trigger immediate daemon data refresh via DBus
refresh:
	busctl --user call io.github.amadeu01.AiUsageIndicator /io/github/amadeu01/AiUsageIndicator io.github.amadeu01.AiUsageIndicator Refresh
	@echo "Refresh triggered"

## status        — show current provider data from the daemon
status:
	busctl --user call io.github.amadeu01.AiUsageIndicator /io/github/amadeu01/AiUsageIndicator io.github.amadeu01.AiUsageIndicator GetProviderData


# ── Stop ─────────────────────────────────────────────────────────

## stop          — stop daemon (systemd service + any direct process)
stop:
	-systemctl --user stop $(SERVICE_FILE) 2>/dev/null
	-pkill -TERM -x $(BINARY) 2>/dev/null
	@echo "Stopped $(BINARY)"

# ── Uninstall ────────────────────────────────────────────────────

## uninstall     — remove daemon and extension
uninstall: uninstall-daemon uninstall-ext

## uninstall-daemon — stop service; remove binary and service file
uninstall-daemon: stop
	-systemctl --user disable $(SERVICE_FILE) 2>/dev/null
	rm -f $(INSTALL_BIN)
	rm -f $(SERVICE_DIR)/$(SERVICE_FILE)
	systemctl --user daemon-reload
	@echo "Daemon uninstalled"

## uninstall-ext — disable + remove GNOME extension
uninstall-ext:
	-gnome-extensions disable $(EXT_UUID) 2>/dev/null
	rm -rf $(EXT_INSTALL)
	@echo "Extension uninstalled"

# ── Clean ────────────────────────────────────────────────────────

## clean         — remove all build artifacts (Rust + TypeScript)
clean: clean-daemon clean-ext

## clean-daemon  — remove Rust build artifacts (target/)
clean-daemon:
	cargo clean

## clean-ext     — remove TypeScript build output (ts-src/dist/)
clean-ext:
	cd $(EXT_SRC) && $(PNPM) run clean

# ── Help ─────────────────────────────────────────────────────────

## help          — list all targets with descriptions
help:
	@echo "ai-usage-indicator build targets:"
	@echo ""
	@grep -E '^## ' Makefile | sed 's/## /  make /'
