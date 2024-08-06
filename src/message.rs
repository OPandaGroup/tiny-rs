use crate::state::{log_text_state::LogText, page::Page};

#[derive(Debug, Clone)]
pub enum Thing {
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
    TurnTo(Page),
    Exit,
    Add(Thing),
    ClearPath,
    Convert,
    ToggleTheme,
    Display(LogText),
}
