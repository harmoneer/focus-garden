# Project Overview
Focus Garden is a terminal-based Pomodoro productivity app written in Rust. It uses ratatui for rendering the TUI, crossterm for input and terminal backend, and tui-big-text for a large timer display. Users complete focus sessions, and each completed session grows a plant through multiple stages on a Canvas. Once a plant reaches its final stage, a new plant begins to grow.

The UI features tabs for Timer, Plant Growth, Statistics, and Settings, with a theme system supporting System (neutral), Rose Pine Light, and Rose Pine Dark modes. Themes use foreground colors only, to respect the user's terminal background. Blocks with icons, Canvas for dynamic plants, Gauges for progress bars, and Sparklines for session trends are used to create a visually engaging experience. Application state is persisted to a JSON file, tracking current plant stage, completed plants, session stats, and user settings.

# Project Goals
Focus Garden aims to provide a distraction-free terminal Pomodoro experience with large, visually appealing plant growth animations, a big timer display, and customizable themes. It tracks total sessions, streaks, and completed plants, provides a Sparkline visualization of recent activity, and supports session sequencing with auto-run sets. Keyboard navigation is fully implemented, including direct tab switching (1–4 keys) and block-based UI navigation. The application should be packaged as both a binary and a library and published to crates.io.

# Application Architecture
## Core Modules
- app.rs holds the global application state, including the active tab, timer state, current plant, garden history, statistics, settings, theme, selected indices, and UI focus states. It exposes methods to initialize the app, update state per tick, and handle input events.
- timer.rs handles Pomodoro logic, including focus sessions, short breaks, long breaks, and custom sequences. Timer states include Idle, Running, Paused, and Finished. Methods include start, pause, resume, stop, reset, and tick. Timer progress is measured in seconds and triggers plant growth upon completion.
- plant.rs represents a single plant being grown on Canvas. Each plant has a stage, growth points for the Canvas, and a maximum stage. Growth stages are Seed, Sprout, Flower, and Mini Tree. Each completed focus session adds points to the plant’s growth_points vector. Stage templates with predefined Canvas coordinates ensure consistent growth.
- garden.rs stores fully grown plants and tracks historical data, streaks, and timestamps.
- theme.rs defines the theme system with System, Rose Pine Light, and Rose Pine Dark variants, providing foreground-only colors for UI elements.

The ui/ directory contains per-tab renderers: timer_ui.rs, plant_ui.rs, stats_ui.rs, and settings_ui.rs. Each renderer uses ratatui widgets, tui-big-text for the timer, Canvas for plants, and theme-aware styling.
- input.rs handles keyboard bindings. Keys 1–4 switch tabs directly. Left/Right arrows switch between dual blocks (Timer: session selection / auto-run set; Settings: setting selection / adjustment). Up/Down arrows select or adjust items. Space starts/pauses/resumes the timer, S stops, R resets, Q quits, Enter adds to auto-run, and Del removes items.
- storage.rs manages JSON persistence using serde and serde_json. It saves and loads the current plant stage, growth points, completed plants, session totals, streaks, and user settings (focus duration, break durations, theme).

# UI Layout Specification
The global layout is a vertical split with a header containing tabs, a main content area, and a footer showing status and help hints. The UI uses foreground-only colors for theme compatibility.

Tabs include Timer [1] ⏳, Plant [2] 🌱, Stats [3] 📊, and Settings [4] ⚙️. Tabs are switched with keys 1–4, and the active tab is highlighted using the Tabs widget.

## Timer Tab
The Timer Tab contains a Block titled "Focus Timer" with a large, centered timer display using tui-big-text. A Gauge widget shows session progress, with colors indicating state: gold while running, rose when paused, and love when finished. Session status is displayed below the gauge. Below the timer, dual blocks allow session selection: left for base sessions (Focus, Short Break, Long Break) and right for the auto-run set with add/delete functionality. Navigation uses Left/Right to switch blocks and Up/Down to select/add/adjust items. Auto-run sequences execute continuously.

## Plant Tab
The Plant Tab contains a Block with a dynamic icon in the title representing the plant stage: 🌱 Seed, 🌿 Sprout, 🌸 Flower, 🌳 Mini Tree. A Canvas widget renders the plant using predefined coordinates for stem, leaves, and blossom. A Gauge shows stage progress, and a Paragraph displays remaining sessions until the next stage. Another Paragraph shows the total number of fully grown plants. Plant growth points update with each completed focus session, with optional subtle animations or color fade effects.

## Stats Tab
The Stats Tab contains a Block titled "Statistics". Paragraphs show total focus sessions, total minutes focused, fully grown plants, current streak, and longest streak. A Sparkline widget visualizes recent session counts, with color indicating performance trends.

## Settings Tab
The Settings Tab uses dual blocks: left for selecting settings (Focus Duration, Short Break, Long Break, Theme), right for adjusting values. Durations are adjustable in minutes (1–60). Theme cycles through System, Rose Pine Light, and Rose Pine Dark. Navigation mirrors the Timer Tab: Left/Right switch blocks, Up/Down select/adjust items. Adjustments autosave when returning to the left block. Focus state (SettingsFocus) controls which block is active.

# Plant Growth Rules
Each plant grows through four stages: Seed, Sprout, Flower, and Mini Tree. Each completed Pomodoro session adds points to the Canvas for the current plant stage. When the final stage is completed, the plant is added to the Garden History, and a new plant begins at the Seed stage. Stage templates with Canvas coordinates define consistent growth patterns.

# Persistence
Application state is saved in $CONFIG_DIR/focus-garden/data.json. Saved fields include the current plant stage, Canvas growth points, completed plants, session totals, streaks, and user settings (focus duration, break durations, theme). On startup, the JSON restores the previous state.

# Event Loop
The main event loop runs at ~250ms per tick. On each tick, the timer decrements if running, plant growth points animate, and the UI is redrawn. Input events are processed asynchronously. Completed sessions trigger plant growth updates. Dual-block focus is handled per-tab, with Up/Down for selection or adjustment and Left/Right for switching blocks. Theme changes are applied dynamically.

# Directory Layout
src/
  ├─ main.rs
  ├─ app.rs
  ├─ input.rs
  ├─ timer.rs
  ├─ plant.rs
  ├─ garden.rs
  ├─ storage.rs
  ├─ theme.rs
  ├─ ui/
  │    ├─ mod.rs
  │    ├─ timer_ui.rs
  │    ├─ plant_ui.rs
  │    ├─ stats_ui.rs
  │    └─ settings_ui.rs


# Publishing Requirements
The crate name is focus-garden. Provide both a binary and an optional library. Include README.md, LICENSE, CHANGELOG.md, and spec.md. Dependencies include ratatui, crossterm, serde, serde_json, chrono, dirs, tui-big-text. Crates.io metadata should include categories command-line-utilities and productivity, and keywords pomodoro, ratatui, tui, focus, plants, productivity, terminal.