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
    WarnText(String),
    ToggleButtonStyle,
    ToggleTheme,
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
