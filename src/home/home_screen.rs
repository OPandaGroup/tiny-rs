use makepad_widgets::*;
use tinify::sync::Tinify;

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::thread;

use crate::app::DIR;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    HomeScreen = {{HomeScreen}} {
        // debug: true
        visible: true
        width: Fill, height: Fill
        align: {x: 0.5}
        padding: {top:5}
        spacing: 20

        pictures = <View> {
            // debug: true

            flow: Down
            align: {x: 0.5}
            spacing: 15
            <View> {
                height: 45
                spacing: 15
                button0 = <Button> {
                    width: Fill, height: 40
                    text: "Upload Folder"
                    draw_text: {
                        color: #77BF77
                        text_style: {
                            font_size: 17.
                        }
                    }
                }
                button1 = <Button> {
                    width: Fill, height: 40
                    text: "Upload Pictures"
                    draw_text: {
                        color: #77BF77
                        text_style: {
                            font_size: 17.
                        }
                    }
                }
            }
            PictureDirs = <Label> {
                text: "No Pictures Yet"
                draw_text: {
                    color: #BBBBBB
                    text_style: {
                        font_size: 20
                    }
                }
            }
            portal_list = <PortalList> {
                keep_invisible: false
                flow: Down
                width: Fill, height: Fill
                // pictures_dirs = <PictureDirs>
            }
        }

        <View> {
            flow: Down
            api_key = <View> {
                height: 200
                // debug: true
                spacing: 15
                text_input = <TextInput> {
                    width: Fill
                    empty_message: "Type your API key here"
                    draw_text: {
                        color: #333333
                        text_style: {
                            font_size: 20.
                        }
                    }
                }
                button = <Button> {
                    width: 58
                    text: "OK!"
                    draw_text: {
                        color: #0
                        text_style: {
                            font_size: 17.
                        }
                    }
                }
            }

            convert = <View> {
                // debug: true

                width: Fill, height: Fill
                align: {x: 0.5, y: 0.5}

                button = <Button> {
                    text: "Convert"
                    draw_text: {
                        color: #FFFFFF
                        text_style: {
                            font_size: 40.
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct HomeScreen {
    #[deref]
    view: View,
    #[rust]
    api_key: String,
    #[rust]
    picture_dirs: HashSet<PathBuf>,
}
impl Widget for HomeScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.view.handle_event(cx, event, scope)
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
impl MatchEvent for HomeScreen {
    fn handle_actions(&mut self, _: &mut Cx, actions: &Actions) {
        if self.button(id!(api_key.button)).clicked(actions) {
            self.api_key = self.text_input(id!(api_key.text_input)).text();
            self.text_input(id!(api_key.text_input)).set_text("")
        }

        if self.button(id!(pictures.button0)).clicked(actions) {
            if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                self.picture_dirs = collect_images_path(&dir)
            }
        }

        if self.button(id!(pictures.button1)).clicked(actions) {
            if let Some(dir) = rfd::FileDialog::new()
                .add_filter("pictures", &["png", "jpg", "jpeg", "JPG", "PNG", "JPEG"])
                .pick_files()
            {
                self.picture_dirs = dir.into_iter().collect()
            }
        }

        if !self.picture_dirs.is_empty()
            && !self.api_key.is_empty()
            && self.button(id!(convert.button)).clicked(actions)
        {
            let api_key = self.api_key.clone();
            let picture_dirs = self.picture_dirs.clone();
            self.picture_dirs.clear();

            thread::spawn(move || {
                for dir in picture_dirs.into_iter() {
                    Tinify::new()
                        .set_key(&api_key)
                        .get_client()
                        .unwrap()
                        .from_file(&dir)
                        .unwrap()
                        .to_file(format!(
                            "{}/{}",
                            DIR.clone(),
                            dir.file_name().unwrap().to_str().unwrap()
                        ))
                        .unwrap();
                }
            });
        }
    }
}

pub fn collect_images_path(path: &PathBuf) -> HashSet<PathBuf> {
    let iter1 = match fs::read_dir(path) {
        Ok(read_dir) => read_dir
            .map(|entry| entry.unwrap().path())
            .filter(|item| {
                item.is_file()
                    && (item.to_str().unwrap().ends_with(".jpg")
                        || item.to_str().unwrap().ends_with(".png"))
            })
            .collect(),
        Err(_) => Vec::new(),
    };

    let iter2 = match fs::read_dir(path) {
        Ok(read_dir) => read_dir
            .map(|entry| entry.unwrap().path())
            .filter(|item| item.is_dir())
            .flat_map(|item| collect_images_path(&item).into_iter())
            .collect(),
        Err(_) => Vec::new(),
    };
    iter1.into_iter().chain(iter2).collect()
}
