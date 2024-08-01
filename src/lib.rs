use std::path::PathBuf;

use iced::widget::{column, container, Button, Text, TextInput};
use iced::{executor, Application, Command};
use images_path::collect_images_path;
use message::AddSth;
use process::process_images;
use rfd::FileDialog;
use tinify::async_bin::Tinify;
pub mod process;
use self::message::Message;
pub mod images_path;
pub mod message;
pub struct App {
    paths: Vec<PathBuf>,
    api_key_val: String,
    warn_text: String,
}
impl App {
    fn clear_images_path(&mut self) {
        self.paths = Vec::new()
    }
    fn rfd_again(&mut self) {
        let paths = collect_images_path(FileDialog::new().pick_folder().unwrap_or_default());
        let iter1 = paths.into_iter();
        let iter2 = self.paths.clone().into_iter();
        self.paths = iter1.chain(iter2).collect();
    }
}
impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::Theme;
    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let paths = Vec::new();
        let api_key_val = "".to_string();
        let warn_text = "".to_string();
        (
            Self {
                paths,
                api_key_val,

                warn_text,
            },
            iced::Command::none(),
        )
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::AddSth(addsth) => {
                match addsth {
                    AddSth::APi(api_key) => {
                        self.api_key_val = api_key;
                    }
                    AddSth::Path => self.rfd_again(),
                }
                Command::none()
            }

            Message::Convert => {
                let path = self.paths.clone();
                let tinify = Tinify::new().set_key(&self.api_key_val);

                Command::perform(
                    async move {
                        process_images(
                            tinify,
                            path.iter()
                                .map(|p| (*p.to_string_lossy()).to_owned())
                                .collect(),
                        )
                        .await
                    },
                    |result| {
                        if let Err(e) = result {
                            Message::WarnText(String::from("Incorrect API Key"))
                        } else {
                            Message::WarnText("".to_string())
                        }
                    },
                )
            }
            Message::ClearPath => {
                self.clear_images_path();
                Command::none()
            }
            Message::WarnText(warn) => {
                self.warn_text = warn;
                Command::none()
            }
        }
    }

    fn title(&self) -> String {
        "tiny-rs".to_string()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let column = column!(
            TextInput::new("API Key", &self.api_key_val)
                .on_input(|s: String| Message::AddSth(AddSth::APi(s)))
                .on_paste(|s: String| Message::AddSth(AddSth::APi(s)))
                .padding(25),
            Button::new("AddPath").on_press(Message::AddSth(AddSth::Path)),
            Button::new("ClearPath").on_press(Message::ClearPath),
            Button::new("Process").on_press(Message::Convert),
            Text::new(self.warn_text.clone()).size(30)
        );

        container(column).into()
    }
}
