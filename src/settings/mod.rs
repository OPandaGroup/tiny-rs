use makepad_widgets::Cx;

pub mod settings_content;
pub mod settings_screen;
pub mod settings_sidebar;

pub fn live_design(cx: &mut Cx) {
    settings_screen::live_design(cx);
    settings_sidebar::live_design(cx);
    settings_content::live_design(cx);
}
