use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    SettingsSidebarButton = <RadioButton> {
        // draw_radio: {
        //     radio_type: Tab
        // }
        draw_text: {
            color: #F0F0F0
            text_style: {
                font_size: 14.
            }
        }
    }
}
