use iced::Color;

pub mod button_style_toggle;
#[derive(Debug, Clone)]
pub enum AddSth {
    APi(String),
    Path,
}

#[derive(Debug, Clone)]
pub enum ClearSth {
    APi,
    Path,
}
#[derive(Debug, Clone)]
pub enum Message {
    AddSth(AddSth),
    ClearPath,
    Convert,
    ToggleButtonStyle,
    ToggleTheme,
    LogText(LogText),
}

#[derive(Debug, Clone)]
pub enum ButtonStyle {
    Standard,
    Lovely,
}
#[derive(Debug, Clone)]
pub enum ThemeTo {
    Light,
    Dark,
    Moonfly,
    Oxocarbon,
}
const SUCCESS: &str = "Convert Success";
const FAIL: &str = "Incorrect APIkey";

#[derive(Debug, Clone)]

pub enum LogText {
    Fail,
    Success,
    Null,
}
impl From<&LogText> for &str {
    fn from(logtext: &LogText) -> Self {
        match logtext {
            LogText::Success => SUCCESS,
            LogText::Fail => FAIL,
            LogText::Null => "",
        }
    }
}
impl From<&LogText> for Color {
    fn from(logtext: &LogText) -> Self {
        match logtext {
            LogText::Success => Color::from_rgb8(0, 220, 0),
            LogText::Fail => Color::from_rgb8(220, 0, 0),
            LogText::Null => Color::default(),
        }
    }
}
