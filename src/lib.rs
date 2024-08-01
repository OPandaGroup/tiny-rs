use std::path::PathBuf;

use iced::widget::text::Shaping;
use iced::widget::{column, container, row, Button, Text, TextInput};
use iced::{executor, theme, Application, Command, Theme};
use images_path::collect_images_path;
use message::{AddSth, ButtonStyle, LogText};
use process::process_images;
use rfd::FileDialog;
use tinify::async_bin::Tinify;

pub mod process;
use self::message::Message;
pub mod images_path;
pub mod message;
use iced::Alignment;
pub struct App {
    paths: Vec<PathBuf>,
    api_key: String,
    log_text: LogText,
    theme: iced::Theme,
    button_style: ButtonStyle,
}
impl App {
    fn clear_images_path(&mut self) {
        self.paths.clear()
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
        let api_key = "".to_string();
        let log_text = LogText::Null;
        let theme = iced::Theme::Moonfly;
        let button_style = ButtonStyle::Standard;
        (
            Self {
                button_style,
                theme,
                paths,
                api_key,
                log_text,
            },
            iced::Command::none(),
        )
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::AddSth(addsth) => {
                self.log_text = LogText::Null;
                match addsth {
                    AddSth::APi(api_key) => {
                        self.api_key = api_key;
                    }
                    AddSth::Path => self.rfd_again(),
                }
                Command::none()
            }

            Message::Convert => {
                let path = self.paths.clone();
                let tinify = Tinify::new().set_key(&self.api_key);

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
                    |result| match result.is_ok() {
                        true => Message::LogText(LogText::Success),
                        false => Message::LogText(LogText::Fail),
                    },
                )
            }
            Message::ClearPath => {
                self.clear_images_path();
                Command::none()
            }

            Message::ToggleTheme => {
                match self.theme.clone() {
                    Theme::Light => self.theme = Theme::Dark,
                    Theme::Dark => self.theme = Theme::Moonfly,
                    Theme::Moonfly => self.theme = Theme::Oxocarbon,
                    Theme::Oxocarbon => self.theme = Theme::Light,
                    _ => self.theme = Theme::default(),
                }
                Command::none()
            }
            Message::ToggleButtonStyle => {
                match self.button_style {
                    ButtonStyle::Standard => self.button_style = ButtonStyle::Lovely,
                    ButtonStyle::Lovely => self.button_style = ButtonStyle::Standard,
                };
                Command::none()
            }
            Message::LogText(log_text) => {
                self.log_text = log_text;
                Command::none()
            }
        }
    }

    fn title(&self) -> String {
        "TinyRS".to_string()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let api_input = container(
            TextInput::new("Type APIKey Here", &self.api_key)
                .on_input(|s: String| Message::AddSth(AddSth::APi(s)))
                .on_paste(|s: String| Message::AddSth(AddSth::APi(s)))
                .padding(25),
        )
        .padding(60)
        .center_x();
        let log_text: &str = (&self.log_text).into();
        let log_text = Text::new(log_text)
            .style(theme::Text::Color((&self.log_text).into()))
            .size(25);

        let basic = container(
            row!(
                Button::new(Text::new("AddPath").shaping(Shaping::Advanced).size(20))
                    .on_press(Message::AddSth(AddSth::Path))
                    .style(theme::Button::custom(self.button_style.clone())),
                Button::new(Text::new("ClearPath").shaping(Shaping::Advanced).size(20))
                    .on_press(Message::ClearPath)
                    .style(theme::Button::custom(self.button_style.clone())),
                Button::new(Text::new("Process").shaping(Shaping::Advanced).size(20))
                    .on_press(Message::Convert)
                    .style(theme::Button::custom(self.button_style.clone())),
            )
            .spacing(8),
        )
        .padding(40)
        .center_x()
        .center_y();

        let settings = container(
            column!(
                Button::new(Text::new("ToggleTheme").shaping(Shaping::Advanced).size(20))
                    .on_press(Message::ToggleTheme)
                    .style(theme::Button::custom(self.button_style.clone())),
                Button::new(
                    Text::new("ToggleButtonStyle")
                        .shaping(Shaping::Advanced)
                        .size(20)
                )
                .on_press(Message::ToggleButtonStyle)
                .style(theme::Button::custom(self.button_style.clone())),
            )
            .align_items(Alignment::Center)
            .spacing(10),
        )
        .padding(40);

        column!(api_input, log_text, basic, settings)
            .spacing(10)
            .align_items(Alignment::Center)
            .into()
    }
    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}
