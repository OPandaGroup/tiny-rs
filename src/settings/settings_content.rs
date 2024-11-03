use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    SettingsContent = <View> {
        // debug: true
        // visible: false
        flow: Down
        width: Fill, height: Fill
        align: {x: 0.5, y: 0.5}
        spacing: 10
        appearance = <View> {
            // debug: true
            visible: false
            width: Fill, height: Fill
            align: {x: 0.5, y: 0.5}
            <Label> {
                text: "Appearance"
                draw_text: {
                    color: #00FFAA
                    text_style: {
                        font_size: 40.
                    }
                }
            }
        }
        notifications = <View> {
            // debug: true
            visible: false
            width: Fill, height: Fill
            align: {x: 0.5, y: 0.5}
            <Label> {
                text: "Notifications"
                draw_text: {
                    color: #00FFAA
                    text_style: {
                        font_size: 40.
                    }
                }
            }
        }
        preferences = <View> {
            // debug: true
            flow: Down
            visible: false
            width: Fill, height: Fill
            align: {x: 0.5}
            spacing: 200
            <Label> {
                text: "Preferences"
                draw_text: {
                    color: #00FFAA
                    text_style: {
                        font_size: 40.
                    }
                }
            }
            <View> {
                align: {x: 0.5}
                width: Fill, height: Fill
                spacing: 10
                <Label> {
                    text: "Choose Your Language"
                    draw_text: {
                        color: #99FF99
                        text_style: {
                            font_size: 19
                        }
                    }
                }
                <DropDown> {
                    labels: ["C", "C++", "Julia", "Zig", "Rust", "ObjC"]
                    values: [A, B, C, D, E, F]
                    draw_text: {
                        text_style: {
                            font_size: 20
                        }
                    }
                }
            }
        }
    }
}
