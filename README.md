# 🤖 GNOME AI Usage Indicator

A GNOME Shell extension and system tray application that monitors AI coding assistant usage in real-time from your top panel.

## 📖 Overview

**AI Usage Indicator** provides real-time visibility into your AI coding assistant consumption across multiple providers:

- 🟣 **Anthropic API** - Direct Claude API usage
- 🔵 **Claude Code** - CLI-based Claude usage
- ⚫ **Codex** - OpenAI Codex integration
- 🔌 **Extensible** - Easy to add new providers

Built with **Rust** 🦀 and **GTK4/Libadwaita** for native GNOME integration.

## ✨ Features

- 📊 Real-time usage monitoring with percentage indicators
- 🎯 System tray integration with status icon
- 🖥️ Detailed usage breakdown per provider
- 🔄 Auto-refresh with configurable intervals
- ⚡ SIGHUP support for config reloading without restart
- 🎨 Native GNOME look and feel
- 🌙 Dark/Light theme support

## 🏗️ Architecture

```
ai-usage-indicator/
├── src/              # Rust GTK4 application
│   ├── main.rs       # Application entry point
│   ├── window.rs     # Main window UI
│   ├── tray.rs       # System tray integration
│   ├── config.rs     # Configuration management
│   ├── providers/    # AI provider implementations
│   └── ui/           # UI components
├── ts-src/           # GNOME Shell Extension (TypeScript)
│   ├── src/          # Extension source
│   ├── schemas/      # GSettings schemas
│   └── res/          # Resources (icons, styles)
└── openspec/         # OpenSpec workflow artifacts
```

## 🚀 Installation

### From Source

```bash
# Build the Rust application
cargo build --release

# Build the GNOME Shell Extension
cd ts-src
npm install
npm run build

# Install the extension
make install
```

### Requirements

- Rust 1.70+ with edition 2024
- GTK4 and Libadwaita
- GNOME Shell 45+
- Node.js and npm (for extension)

## ⚙️ Configuration

Create `~/.config/ai-usage-indicator/config.toml`:

```toml
# Polling interval in seconds
poll_interval_secs = 30

# Provider configurations
[providers.anthropic]
enabled = true
api_key = "your-api-key"

[providers.claude_code]
enabled = true
log_path = "~/.claude/projects/**/*.jsonl"

[providers.codex]
enabled = true
```

## 🤖 AI-First Development

This project is developed with **AI as a first citizen**:

- 📝 All code changes documented with OpenSpec workflow
- 🔄 AI-assisted development throughout the codebase
- 📚 AI context preserved in `.pi/` and `openspec/` directories
- 🎯 Designed for human-AI collaboration

### OpenSpec Workflow

This project uses the OpenSpec methodology for AI-assisted development:

```bash
# Propose a new feature
pi propose "Add provider X support"

# Explore design decisions
pi explore "How should we handle rate limiting?"

# Apply changes from a spec
pi apply-change <change-id>

# Archive completed changes
pi archive-change <change-id>
```

## 📊 Usage

1. Start the application: `cargo run --release`
2. Check the system tray for usage indicator
3. Click the indicator for detailed breakdown
4. Configure providers in `~/.config/ai-usage-indicator/config.toml`

### GNOME Extension

Enable the extension via GNOME Extensions app or:

```bash
gnome-extensions enable gnome-ai-usage-indicator@amadeu01.github.io
```

## 🔧 Development

```bash
# Run in development mode
cargo run

# Run with logging
RUST_LOG=debug cargo run

# Build release
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## 📁 Project Structure

| Directory | Purpose |
|-----------|---------|
| `src/` | Rust GTK4 application |
| `ts-src/` | GNOME Shell Extension |
| `res/` | Shared resources (icons) |
| `openspec/` | OpenSpec change tracking |
| `.pi/` | Pi agent configuration |
| `.agent/` | Agent-specific configs |

## 🎯 Roadmap

- [ ] Add more AI provider support
- [ ] Historical usage tracking
- [ ] Usage alerts and notifications
- [ ] Export usage reports
- [ ] System-wide AI usage detection

## 📄 License

MIT License - See [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions welcome! Please read our contributing guidelines and follow the OpenSpec workflow for AI-assisted development.

---

**Built with 🦀 Rust, 🖥️ GTK4, and 🤖 AI collaboration**
