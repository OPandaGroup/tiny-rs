use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::settings::settings_sidebar::SettingsSidebar;
    import crate::settings::settings_content::SettingsContent;

    SettingsScreen = <View> {

        visible: false
        flow: Right
        width: Fill, height: Fill

        settings_sidebar = <SettingsSidebar> {}
        settings_content = <SettingsContent> {}
    }
}
