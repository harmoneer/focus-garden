# Taman

🌱 **Taman** is a TUI Pomodoro productivity app where your focus sessions literally grow plants! Built in Rust with Ratatui, it combines the Pomodoro technique with gamification — complete focus sessions to evolve your plant from a 🌰 seed to a 🪴 fully grown plant - all in your terminal!

[![Buy Me a Coffee at ko-fi.com](https://plok.sh/kofi.png)](https://ko-fi.com/A0A81PC8ZA)

## Features
[![Taman Features](https://github.com/harmoneer/taman/blob/master/screenshots/tamanv0.1.3.gif?raw=true)]

- **Pomodoro Timer**: Configurable focus, short break, and long break durations
- **Plant Growth**: Emoji-based plants that grow with each completed session
- **Auto-Run Queue**: Queue multiple sessions to run automatically
- **Statistics & Charts**: Daily metrics with bar charts and pie charts for productivity insights
- **Themes**: Multiple color themes (System, Rose Pine Dawn, Rose Pine, Gruvbox Dark/Light, Solarized Dark/Light, Nord, Tokyo Night, Monokai, Vesper, Everforest, Catppuccin Latte/Frappé/Macchiato/Mocha)
- **Persistence**: JSON-based saving of progress, stats, and settings
- **Keyboard Navigation**: Fully navigable with keyboard only
- **Streak Tracking**: Current and longest daily streaks

## Installation

### From Source

```bash
git clone https://github.com/yourusername/taman.git
cd taman
cargo build --release
./target/release/taman
```

### From Crates.io

```bash
cargo install taman
taman
```

## Usage

Taman is a terminal user interface (TUI) app. Run it and use the following keys:

- **1/2/3/4**: Switch between tabs (Timer, Plant, Stats, Settings)
- **Arrow Keys**: Navigate within tabs
- **Enter**: Select/activate
- **Space**: Start/pause timer
- **p**: Pause/unpause the timer
- **s**: Stop timer
- **q**: Quit

### Tabs

1. **⏳ Timer**: Select session type with arrows, start with Space, stop with s, manage auto-run queue
2. **🌿 Plant**: View current plant stage and progress
3. **📊 Stats**: Browse daily metrics and charts with arrow navigation
4. **⚙️ Settings**: Adjust durations and change themes

## Configuration

Settings are saved automatically to `~/.config/taman/data.json` (or equivalent on your OS).

- Focus duration (default: 25 minutes)
- Short break (default: 5 minutes)
- Long break (default: 15 minutes)
- Theme selection

## Dependencies

- Rust 1.70+
- Terminal with Unicode support

## Libraries Used

- [ratatui](https://github.com/ratatui-org/ratatui) - TUI framework
- [tui-big-text](https://github.com/joshka/tui-big-text) - Large text displays
- [tui-piechart](https://github.com/ImJeremyHe/tui-piechart) - Pie charts
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal backend
- [serde](https://github.com/serde-rs/serde) - Serialization

## Contributing

Contributions welcome! Please open issues or pull requests on GitHub.

## License

MIT License

## Devlogs

- [Devlog #1](blog/first.md) - Initial concepts and architecture
- [Devlog #2](blog/second.md) - Implementation progress to working prototype
- [Devlog #3](blog/third.md) - Crates.io release and community launch
- [Devlog #4](blog/fourth.md) - Expanded theme support

---

🌿 Grow your focus, grow your garden.
