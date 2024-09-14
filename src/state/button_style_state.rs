use iced::{widget::button, Border};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonColor {
    #[serde(default = "default_background_color")]
    background: String,
    #[serde(default = "default_text_color")]
    text: String,
    #[serde(default = "default_border_color")]
    border: String,
}

impl Default for ButtonColor {
    fn default() -> Self {
        let background = default_background_color();
        let text = default_text_color();
        let border = default_border_color();
        Self {
            background,
            text,
            border,
        }
    }
}

fn default_background_color() -> String {
    String::from("#eeeeee")
}

fn default_text_color() -> String {
    String::from("#111111")
}

fn default_border_color() -> String {
    String::from("#000000")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonColorState {
    #[serde(default)]
    onpressed: ButtonColor,
    #[serde(default)]
    unpressed: ButtonColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonStyle {
    #[serde(default)]
    pub radius: u8,
    #[serde(default)]
    pub color: ButtonColorState,
}

impl Default for ButtonColorState {
    fn default() -> Self {
        ButtonColorState {
            onpressed: ButtonColor {
                background: "#00ee00".to_string(),
                text: "#eeeeee".to_string(),
                border: "#000000".to_string(),
            },
            unpressed: ButtonColor {
                background: "#333333".to_string(),
                text: "#eeeeee".to_string(),
                border: "#000000".to_string(),
            },
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        let radius = 10;
        let color = ButtonColorState::default();
        Self { radius, color }
    }
}

impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(hex_to_color(
                &self.color.unpressed.background,
            ))),
            text_color: hex_to_color(&self.color.unpressed.text),
            border: Border {
                color: hex_to_color(&self.color.unpressed.border),
                radius: self.radius.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(hex_to_color(
                &self.color.onpressed.background,
            ))),
            text_color: hex_to_color(&self.color.onpressed.text),
            border: Border {
                color: hex_to_color(&self.color.onpressed.border),
                radius: self.radius.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

fn hex_to_color(hex: &str) -> iced::Color {
    let (r, g, b) = hsluv::hex_to_rgb(hex);
    iced::Color::from_rgb(r as f32, g as f32, b as f32)
}
