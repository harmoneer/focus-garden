# 🌱 Taman Devlog #4 — Expanded Theme Support

Welcome back to **Taman**, the Rust TUI Pomodoro app where your focus sessions literally grow plants! In [Devlog #3](https://plok.sh/harmoneer/taman/third), we celebrated the crates.io release of Taman v0.1.0. Now, we're excited to share Taman v0.1.3, which brings a massive expansion in theme options to make your productivity garden even more personalized!

This devlog covers the addition of 13 new themes, theme management improvements, and our vision for future customization.

[![Buy Me a Coffee at ko-fi.com](https://plok.sh/kofi.png)](https://ko-fi.com/A0A81PC8ZA)

---

## 🌿 Recap: From Release to Themes

In Devlog #3, we launched Taman v0.1.0 on crates.io with:

- Core Pomodoro functionality
- Plant growth visualization
- Statistics and charts
- Basic theme support (System, Rose Pine Dawn, Rose Pine)

Since the release, community feedback highlighted the desire for more visual customization. We responded by expanding theme support to include popular color schemes, ensuring Taman adapts to your preferred aesthetic.

---

## 🎨 New Theme Collection

Taman v0.1.3 introduces **13 new themes**, bringing the total to **16 themes**:

- **Classic Themes**: Gruvbox Dark/Light, Solarized Dark/Light, Nord
- **Modern Themes**: Tokyo Night, Monokai, Vesper
- **Nature-Inspired**: Everforest
- **Catppuccin Flavors**: Latte, Frappé, Macchiato, Mocha

Each theme uses foreground-only colors for compatibility with your terminal background, and all UI elements (borders, text, gauges) are consistently themed.

### How to Switch Themes

In Taman, press `4` to enter Settings, navigate to the Theme option, and use left/right arrows to cycle through themes. Changes apply instantly!

---

## 🔧 Technical Improvements

- **Theme Variants**: Added 13 new `ThemeVariant` enums with carefully mapped RGB colors.
- **UI Consistency**: Ensured big text, borders, and elements match theme colors across tabs.
- **Backward Compatibility**: Old theme names are aliased for smooth upgrades.

---

## 🌻 What's Coming Next?

- Enhanced customization (custom colors, layouts)
- Sound notifications for session changes
- Integration with external tools
- Mobile/web companion apps

---

## 🌻 Closing Thoughts

Taman v0.1.3 makes your focus garden bloom in your favorite colors! Whether you prefer the warmth of Gruvbox or the coolness of Nord, there's a theme for everyone. Update with `cargo install taman --force` and explore the new themes.

If you have theme suggestions or feedback, let us know on GitHub!

🌿 Happy theming!