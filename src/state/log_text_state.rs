use iced::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogText {
    Fail,
    Success,
    Null,
}
const SUCCESS: &str = "Convert Success";
const FAIL: &str = "Incorrect APIkey";
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
