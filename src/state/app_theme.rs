use iced::Theme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppTheme {
    Light,
    Dark,
    Moonfly,
    Oxocarbon,
}
impl Default for AppTheme {
    fn default() -> Self {
        AppTheme::Light
    }
}
impl From<AppTheme> for Theme {
    fn from(app_theme: AppTheme) -> Self {
        match app_theme {
            AppTheme::Light => Theme::Light,
            AppTheme::Dark => Theme::Dark,
            AppTheme::Moonfly => Theme::Moonfly,
            AppTheme::Oxocarbon => Theme::Oxocarbon,
        }
    }
}
