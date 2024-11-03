use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    DockButton = <RadioButton> {
        // debug: true
        width: Fill, height: 80

        draw_radio: {
            radio_type: Tab
        }

        draw_icon: {
            fn get_color(self) -> vec4 {
                return #FFAAFF;
            }
        }

        icon_walk: {width: 35, height: Fit}
    }
}
