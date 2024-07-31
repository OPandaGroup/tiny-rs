use iced::widget::text_editor::Content;
use std::fs;
use std::path::PathBuf;

// use iced::widget::text_editor::Content;
// use iced::theme::TextEditor;
use iced::widget::{column, container, Button, Text, TextInput};
use iced::{executor, Application, Command, Settings};
use rfd::FileDialog;
use tinify::async_bin::Tinify;
// use tinify::error::TinifyError;
struct App {
    path: Vec<PathBuf>,
    api_key_val: String,
    tinify: Tinify,
    // api_key_is_ok: bool,
    content: Content,
    warn_text: Option<String>,
}
#[derive(Debug, Clone)]
enum State {
    Start,
    Ing,
    Finish,
}

#[derive(Debug, Clone)]
enum AddSth {
    APi(String),
    Path,
}

#[derive(Debug, Clone)]
enum ClearSth {
    APi(State),
    Path(State),
}
#[derive(Debug, Clone)]
enum Message {
    AddSth(AddSth),
    ClearPath,
    Convert,
}
impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::Theme;
    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let path = Vec::new();
        let tinify = Tinify::new();
        // let api_key_is_ok = false;
        let content = Content::new();
        (
            Self {
                content,
                path,
                api_key_val: "".into(),
                tinify,
                // api_key_is_ok,
                warn_text: None,
            },
            iced::Command::none(),
        )
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::AddSth(addsth) => {
                match addsth {
                    AddSth::APi(api_key) => {
                        self.tinify = Tinify::new().set_key(api_key);
                    }
                    AddSth::Path => {
                        let paths = image_files(
                            FileDialog::new().pick_folder().unwrap(), //todo: handle Err Better?
                        );
                        let iter1 = paths.into_iter();
                        let iter2 = self.path.clone().into_iter();
                        self.path = iter1.chain(iter2).collect();
                    }
                }
                Command::none()
            }

            Message::Convert => {
                todo!()
            }
            Message::ClearPath => Command::none(),
        }
    }

    fn title(&self) -> String {
        "tiny-rs".to_string()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let column = column!(
            TextInput::new("API Key", &self.api_key_val)
                .on_input(|s: String| Message::AddSth(AddSth::APi(s)))
                .padding(40),
            Button::new("ClearPath").on_press(Message::ClearPath)
        );

        let column = if let Some(text) = &self.warn_text {
            column.push(Text::new(text))
        } else {
            column
        };

        container(column).into()
    }
}

impl App {
    async fn process_image(&self, path: Vec<String>, api_key: &str) {
        for p in path {
            let new_file_name = p
                .split("/")
                .last()
                .unwrap()
                .split(".")
                .to_owned()
                .collect::<Vec<_>>()[0]
                .to_string()
                + "optimized"; // a.png -> a-optimized.png

            self.tinify
                .get_async_client()
                .unwrap()
                .from_file(p.clone())
                .await
                .unwrap()
                .to_file(new_file_name)
                .await;
        }
    }
}

fn image_files(path: PathBuf) -> Vec<PathBuf> {
    let iter1 = fs::read_dir(&path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|item| {
            item.is_file()
                && (item.to_str().unwrap().to_lowercase().ends_with(".jpg")
                    || item.to_str().unwrap().to_lowercase().ends_with(".png")
                    || item.to_str().unwrap().to_lowercase().ends_with(".webp"))
        });

    let iter2 = fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|item| item.is_dir())
        .flat_map(|item| image_files(item).into_iter());
    iter1.chain(iter2).collect()
}

fn main() {
    App::run(Settings::default()).unwrap()
}
