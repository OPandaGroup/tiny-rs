use std::fs;
use std::path::PathBuf;

// use iced::widget::text_editor::Content;
// use iced::theme::TextEditor;
use iced::widget::{column, container, Button, Text, TextInput};
use iced::{executor, Application, Command, Settings};
use rfd::FileDialog;
use tinify::async_bin::Tinify;
// static mut
// use tinify::error::TinifyError;
struct App {
    path: Vec<PathBuf>,
    api_key_val: String,
    // content: Content,
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
    WarnTextUpdated(Option<String>),
}
impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::Theme;
    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let path = Vec::new();
        (
            Self {
                // content,
                path,
                api_key_val: "".into(),
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
                        self.api_key_val = api_key;
                    }
                    AddSth::Path => {
                        let paths =
                            image_files(FileDialog::new().pick_folder().unwrap_or_default());
                        let iter1 = paths.into_iter();
                        let iter2 = self.path.clone().into_iter();
                        self.path = iter1.chain(iter2).collect();
                    }
                }
                Command::none()
            }

            Message::Convert => {
                let path = self.path.clone();
                let tinify: Tinify = Tinify::new().set_key(&self.api_key_val);

                Command::perform(
                    async move {
                        process_image(
                            tinify,
                            path.iter()
                                .map(|p| (*p.to_string_lossy()).to_owned())
                                .collect(),
                        )
                        .await
                    },
                    |result| {
                        if let Err(e) = result {
                            Message::WarnTextUpdated(Some(e.to_string()))
                        } else {
                            Message::WarnTextUpdated(None)
                        }
                    },
                )
            }
            Message::ClearPath => Command::none(),
            Message::WarnTextUpdated(warn) => {
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
                .padding(40),
            Button::new("AddPath").on_press(Message::AddSth(AddSth::Path)),
            Button::new("ClearPath").on_press(Message::ClearPath),
            Button::new("Process").on_press(Message::Convert)
        );

        let column = if let Some(text) = &self.warn_text {
            column.push(Text::new(text))
        } else {
            column
        };

        container(column).into()
    }
}

async fn process_image(tinify: Tinify, path: Vec<String>) -> anyhow::Result<()> {
    for p in path {
        let mut new_file_name = Vec::new();
        let mut optimized_string_is_added = false;

        p.split(".").for_each(|i| {
            if !i.is_empty() && !optimized_string_is_added {
                new_file_name.push(format!("{i}-optimized"));
                optimized_string_is_added = true;
            } else {
                new_file_name.push(i.to_string());
            }
        });

        let new_file_name = new_file_name.join(".");

        tinify
            .get_async_client()?
            .from_file(p.clone())
            .await?
            .to_file(new_file_name.clone())
            .await?;

        println!("{new_file_name} processed!");
    }
    Ok(())
}

fn image_files(path: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let iter1 = fs::read_dir(&path)?
        .map(|entry| entry.unwrap().path())
        .filter(|item| {
            item.is_file()
                && (item.to_str().unwrap().to_lowercase().ends_with(".jpg")
                    || item.to_str().unwrap().to_lowercase().ends_with(".png")
                    || item.to_str().unwrap().to_lowercase().ends_with(".webp"))
        });

    let iter2 = fs::read_dir(path)?
        .map(|entry| entry.unwrap().path())
        .filter(|item| item.is_dir())
        .flat_map(|item| image_files(item).unwrap().into_iter());
    Ok(iter1.chain(iter2).collect())
}

fn main() {
    App::run(Settings::default()).unwrap()
}
