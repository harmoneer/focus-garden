# 🌱 Taman Devlog #1 — Plant-Powered Pomodoro in Your Terminal

Welcome to the very first development log of **Taman**, a Rust-powered TUI Pomodoro app where **your productivity grows plants** — literally! The idea is simple: every time you complete a focus session, your little plant evolves from a 🌰 seed into a 🌳 mini tree.

This devlog documents the early building blocks of the project: architecture decisions, UI concepts using Ratatui, the swap from ASCII art to emoji plants, and the direction for charts and statistics.  

---

## 🌿 What Is Taman?

Taman is a **terminal-based productivity app** built in Rust using:

- **ratatui** for the TUI widgets  
- **crossterm** for terminal backend + input  
- **serde / JSON** for saving progress  
- **tui-big-text** for large timer digits  

It’s designed to be:

- ✨ **cute** (plants grow!)
- 🧭 **navigable by keyboard** only
- 🎨 **theme-aware** (System / Rose Pine Dawn / Rose Pine / Gruvbox Dark/Light / Solarized Dark/Light / Nord / Tokyo Night / Monokai / Vesper / Everforest / Catppuccin Latte/Frappé/Macchiato/Mocha)
- 📈 **stats-rich**, including charts
- 🪴 **game-ified**, but without distractions  

---

## 🌻 The Core Loop: Grow a Plant by Focusing

Instead of coins, XP, or badges, Taman grows a **single plant at a time**.

Plant lifecycle:

1. 🌰 **Seed**  
2. 🌱 **Sprout**  
3. 🌸 **Flower**  
4. 🌳 **Mini Tree**

Each completed **focus session** advances growth.  
When the plant reaches 🌳, it is archived in the garden, and a fresh 🌰 seed begins.

Plants display as emojis — simple, readable, consistent across terminals.

---

## 🧱 Project Architecture (High-Level)
```
src/
├─ main.rs
├─ app.rs // global app state
├─ input.rs // keyboard handling
├─ timer.rs // pomodoro logic
├─ plant.rs // plant model (emoji-based)
├─ garden.rs // history + streaks
├─ stats.rs // data aggregation
├─ storage.rs // JSON persistence
├─ theme.rs // theme manager
└─ ui/
├─ timer_ui.rs
├─ plant_ui.rs
├─ stats_ui.rs
└─ settings_ui.rs
```


Each tab renders independently and communicates only via `AppState`.

---

## 🎛️ UI Tabs Overview

### 1. ⏳ Timer Tab  
- Large timer via **tui-big-text**  
- Gauge for session progress  
- Blocks for:
  - base session types  
  - auto-run queue  
- Keyboard-driven start/pause/stop  
- Running → increments plant progress  

### 2. 🌿 Plant Tab  
Displays:

- current plant icon (🌰 → 🌱 → 🌸 → 🌳)  
- progress gauge  
- sessions until next stage  
- total plants completed  

This tab is intentionally simple and cute.

### 3. 📊 Stats Tab  
**New metric model (daily):**

- total sessions  
- total minutes  
- focus sessions  
- minutes focused  
- break sessions  
- minutes resting  

We support multiple visualizations:
- sparkline  
- bar chart  
- line chart  
- pie chart (focus vs break)

UI concept:  
Left = metric categories  
Right = chart that updates when selected  

### 4. ⚙️ Settings Tab  
Dual-block layout with:

Left → category  
Right → adjustable values  

Auto-save when you leave the field.

Supports:
- focus duration  
- break durations  
- theme selection  

---

## 🌸 Why Emoji Plants Instead of ASCII Canvas?

We originally tried drawing plants on Ratatui’s Canvas widget.

It worked… but:

- ASCII scaling was inconsistent across terminals  
- Canvas coordinates made early growth look sparse  
- Users running tiny terminals couldn’t see enough detail  
- Emojis are portable, readable, and cute  

So we moved to an emoji model:
```
pub enum PlantStage {
    Seed,       // 🌰
    Sprout,     // 🌱
    Flower,     // 🌸
    Tree,       // 🌳
}
```

And the UI simply renders:
`Current plant: 🌱 Sprout (2 sessions until 🌸)`
Much cleaner. Much happier.

## JSON Persistence Format
```
{
  "current_plant": {
    "stage": "Sprout",
    "sessions": 3
  },
  "garden_history": [
    { "completed_at": "2025-01-05T12:33:00Z" }
  ],
  "stats": {
    "daily": {
      "2025-01-05": {
        "focus_sessions": 3,
        "minutes_focused": 75,
        "break_sessions": 2,
        "minutes_resting": 10
      }
    }
  },
  "settings": {
    "durations": {
      "focus": 25,
      "short_break": 5,
      "long_break": 15
    },
    "theme": "RosePine"
  }
}
```

# 🚧 What’s Coming Next?
- Implementing real charts in the Stats tab
- Adding pie chart breakdowns (focus vs break)
- Garden history page? (mini trees lined up!)
- Polishing block navigation
- Packaging & publishing to crates.io

# 🌻 Closing Thoughts
This first devlog sets the foundation for Taman — the architecture, UI ideas, emoji plant design, and how stats will work. The next entries will focus on implementing real charts, animations, and saving/loading reliable daily metrics.
🌿 Grow your focus, grow your garden.