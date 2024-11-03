use makepad_widgets::Cx;

pub mod dock_button;
pub mod settings_sidebar_button;

pub fn live_design(cx: &mut Cx) {
    dock_button::live_design(cx);
    settings_sidebar_button::live_design(cx);
}
