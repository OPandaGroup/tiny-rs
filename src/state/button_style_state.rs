use iced::{widget::button, Border, Color, Theme};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ButtonStyle {
    Standard,
    Lovely,
}
impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(match style {
                Theme::Light | Theme::Oxocarbon => iced::Color::BLACK,
                Theme::Dark | Theme::Moonfly => iced::Color::WHITE,
                _ => iced::Color::default(),
            })),
            text_color: match style {
                Theme::Light | Theme::Oxocarbon => iced::Color::WHITE,
                Theme::Dark | Theme::Moonfly => iced::Color::BLACK,
                _ => iced::Color::default(),
            },
            border: match self {
                Self::Standard => Border::default(),
                Self::Lovely => Border::with_radius(20),
            },
            ..Default::default()
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(0, 230, 0))),
            text_color: match style {
                Theme::Light | Theme::Oxocarbon => iced::Color::BLACK,
                Theme::Dark | Theme::Moonfly => iced::Color::WHITE,
                _ => iced::Color::default(),
            },
            border: match self {
                Self::Standard => Border::default(),
                Self::Lovely => Border::with_radius(20),
            },
            ..Default::default()
        }
    }
}
