use iced::{widget::button, Border, Theme};
impl button::StyleSheet for super::ButtonStyle {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb8(
                130, 130, 130,
            ))),
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
    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb8(
                100, 100, 100,
            ))),
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
