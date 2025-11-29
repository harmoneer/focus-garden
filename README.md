# Taman

🌱 **Taman** is a terminal-based Pomodoro productivity app where your focus sessions literally grow plants! Built in Rust with Ratatui, it combines the Pomodoro technique with gamification — complete focus sessions to evolve your plant from a 🌰 seed to a 🪴 fully grown plant.

<a href='https://ko-fi.com/A0A81PC8ZA' target='_blank'><img height='36' style='border:0px;height:36px;' src='https://storage.ko-fi.com/cdn/kofi4.png?v=6' border='0' alt='Buy Me a Coffee at ko-fi.com' /></a>

## Features

- **Pomodoro Timer**: Configurable focus, short break, and long break durations
- **Plant Growth**: Emoji-based plants that grow with each completed session
- **Auto-Run Queue**: Queue multiple sessions to run automatically
- **Statistics & Charts**: Daily metrics with bar charts and pie charts for productivity insights
- **Themes**: Multiple color themes (System, Rose Pine Light, Rose Pine Dark)
- **Persistence**: JSON-based saving of progress, stats, and settings
- **Keyboard Navigation**: Fully navigable with keyboard only
- **Streak Tracking**: Current and longest daily streaks

## Screenshots
![Timer Page](https://github.com/harmoneer/taman/blob/master/screenshots/1.%20Timer%20Page.png?raw=true)
![Plant Page](https://github.com/harmoneer/taman/blob/master/screenshots/2.%20Plant%20Page.png)
![Stats Page](https://github.com/harmoneer/taman/blob/2bf27ba6a05787d4e0c4c3b1d4d79ff502a48011/screenshots/3.%20Stats%20Page.png)
![Settings Page](https://github.com/harmoneer/taman/blob/2bf27ba6a05787d4e0c4c3b1d4d79ff502a48011/screenshots/4.%20Settings%20Page.png)

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

---

🌿 Grow your focus, grow your garden.
