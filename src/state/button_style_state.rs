use iced::{widget::button, Border};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonStyle {
    pub radius: u16,

    pub background_color_hex: String,
    pub text_color_hex: String,
    pub border_color_hex: String,

    pub background_color_hex_pressed: String,
    pub text_color_hex_pressed: String,
    pub border_color_hex_pressed: String,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            radius: 3,

            background_color_hex: "#333333".into(),
            text_color_hex: "#ffffff".into(),
            border_color_hex: "#3645ff".into(),

            background_color_hex_pressed: "#3204ff".into(),
            text_color_hex_pressed: "#eeeeee".into(),
            border_color_hex_pressed: "#3204ff".into(),
        }
    }
}

impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        let border = Border {
            color: hex_to_color(&self.border_color_hex),
            radius: self.radius.into(),
            ..Default::default()
        };

        button::Appearance {
            background: Some(iced::Background::Color(hex_to_color(
                &self.background_color_hex,
            ))),
            text_color: hex_to_color(&self.text_color_hex),
            border,
            ..Default::default()
        }
    }

    fn pressed(&self, _: &Self::Style) -> button::Appearance {
        let border = Border {
            color: hex_to_color(&self.border_color_hex_pressed),
            radius: self.radius.into(),
            ..Default::default()
        };

        button::Appearance {
            background: Some(iced::Background::Color(hex_to_color(
                &self.background_color_hex_pressed,
            ))),
            text_color: hex_to_color(&self.text_color_hex_pressed),
            border,
            ..Default::default()
        }
    }
}

fn hex_to_color(hex: &str) -> iced::Color {
    let (r, g, b) = hsluv::hex_to_rgb(hex);
    iced::Color::from_rgb(r as f32, g as f32, b as f32)
}
