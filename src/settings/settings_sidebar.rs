use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::settings_sidebar_button::SettingsSidebarButton;

    SettingsSidebar = <View> {
        debug: true

        flow: Down
        width: 127, height: Fill
        padding: {top: 70}
        spacing: 230

        appearance = <SettingsSidebarButton> {
            text: "Appearance"
        }
        notifications = <SettingsSidebarButton> {
            text: "Notifications"
        }
        preferences = <SettingsSidebarButton> {
            text: "Preferences"
        }
    }
}
