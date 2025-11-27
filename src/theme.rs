use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ThemeVariant {
    System,
    RosePineLight,
    RosePineDark,
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub tabs: Color,
    pub blocks: Color,
    pub text: Color,
    pub secondary_text: Color,
    pub highlight: Color,
    pub timer_text: Color,
    pub gauge_running: Color,
    pub gauge_paused: Color,
    pub gauge_finished: Color,
    pub sparkline: Color,
    pub rose: Color,
    pub love: Color,
    pub foam: Color,
    pub pine: Color,
    pub vertical_value: Color,
}

impl Theme {
    pub fn new(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::System => Theme {
                tabs: Color::White,
                blocks: Color::White,
                text: Color::White,
                secondary_text: Color::Gray,
                highlight: Color::Yellow,
                timer_text: Color::White,
                gauge_running: Color::Yellow,
                gauge_paused: Color::Magenta,
                gauge_finished: Color::Green,
                sparkline: Color::Blue,
                rose: Color::Yellow,
                love: Color::White,
                foam: Color::Yellow,
                pine: Color::White,
                vertical_value: Color::White,
            },
            ThemeVariant::RosePineLight => Theme {
                tabs: Color::Rgb(152, 147, 165),
                blocks: Color::Rgb(152, 147, 165),
                text: Color::Rgb(87, 82, 121),
                secondary_text: Color::Rgb(206, 202, 205),
                highlight: Color::Rgb(144, 122, 169),
                timer_text: Color::Rgb(215, 130, 126),
                gauge_running: Color::Rgb(86, 148, 159),
                gauge_paused: Color::Rgb(40, 105, 131),
                gauge_finished: Color::Rgb(86, 148, 159),
                sparkline: Color::Rgb(234, 157, 52),
                rose: Color::Rgb(235, 188, 186),
                love: Color::Rgb(235, 111, 146),
                foam: Color::Rgb(156, 207, 216),
                pine: Color::Rgb(49, 116, 143),
                vertical_value: Color::Rgb(49, 116, 143),
            },
            ThemeVariant::RosePineDark => Theme {
                tabs: Color::Rgb(57, 53, 82),
                blocks: Color::Rgb(224, 222, 244),
                text: Color::Rgb(224, 222, 244),
                secondary_text: Color::Rgb(82, 79, 103),
                highlight: Color::Rgb(196, 167, 231),
                timer_text: Color::Rgb(156, 207, 216),
                gauge_running: Color::Rgb(234, 154, 151),
                gauge_paused: Color::Rgb(235, 111, 146),
                gauge_finished: Color::Rgb(62, 143, 176),
                sparkline: Color::Rgb(246, 193, 119),
                rose: Color::Rgb(235, 188, 186),
                love: Color::Rgb(235, 111, 146),
                foam: Color::Rgb(156, 207, 216),
                pine: Color::Rgb(49, 116, 143),
                vertical_value: Color::Rgb(49, 116, 143),
            },
        }
    }
}