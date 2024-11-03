use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::dock_button::DockButton;

    ICON_HOME = dep("crate://self/resources/icons/home.svg")
    ICON_SETTINGS = dep("crate://self/resources/icons/settings.svg")

    HomeTab = <DockButton> {
        animator: {selected = {default: on}}
        draw_icon: {
            svg_file: (ICON_HOME)
        }
    }

    SettingsTab = <DockButton> {
        draw_icon: {
            svg_file: (ICON_SETTINGS)
        }
    }

    DockScreen = <View> {
        debug: true
        flow: Down
        width: 70, height: Fill
        padding: {top: 119}
        spacing: 280

        home_tab = <HomeTab> {}
        settings_tab = <SettingsTab> {}
    }
}
