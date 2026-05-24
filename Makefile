BINARY = ai-usage-indicator
INSTALL_BIN = $(HOME)/.local/bin/$(BINARY)
SERVICE_DIR = $(HOME)/.config/systemd/user
SERVICE_FILE = $(BINARY).service

.PHONY: build install run uninstall clean

build:
	cargo build --release

install: build
	install -Dm755 target/release/$(BINARY) $(INSTALL_BIN)
	install -Dm644 $(SERVICE_FILE) $(SERVICE_DIR)/$(SERVICE_FILE)
	install -Dm644 res/icons/$(BINARY)-symbolic.svg $(HOME)/.local/share/icons/hicolor/scalable/apps/$(BINARY)-symbolic.svg
	-gtk-update-icon-cache $(HOME)/.local/share/icons/hicolor/
	systemctl --user daemon-reload
	systemctl --user enable --now $(SERVICE_FILE)

run:
	cargo run

uninstall: 
	-systemctl --user stop $(SERVICE_FILE)
	-systemctl --user disable $(SERVICE_FILE)
	rm -f $(INSTALL_BIN) $(SERVICE_DIR)/$(SERVICE_FILE)
	systemctl --user daemon-reload

clean:
	cargo clean
