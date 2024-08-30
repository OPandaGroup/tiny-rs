use self::message::Message;
use iced::widget::text::Shaping;
use iced::widget::{column, container, row, Button, Text, TextInput};
use iced::{executor, theme, Alignment, Application, Command};
use message::Thing;
use state::app_theme::AppTheme;
use state::log_text_state::LogText;
use state::page::Page;
use state::process_images;
use std::process::exit;
use tinify::async_bin::Tinify;
use tokio::fs;
pub mod images_path;
pub mod message;
pub mod state;
//App里只放std的和自己的数据类型，不放第三方crate的数据类型
pub struct App {
    config: state::Config,
    cache: state::Cache,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = (state::Config, state::Cache);
    type Message = Message;
    type Theme = iced::Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (config, cache) = flags;
        (Self { config, cache }, iced::Command::none())
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Add(thing) => {
                self.cache.log_text = LogText::Null;
                match thing {
                    Thing::APi(api_key) => {
                        self.cache.api_key = api_key;
                    }
                    Thing::Path => self.cache.rfd_again(),
                }
                Command::none()
            }

            Message::Convert => {
                if self.cache.rfd_opened_path.0.is_empty() {
                    return Command::none();
                }
                self.cache.log_text = LogText::Null;
                let mut paths = self.cache.paths.clone();
                let tinify = Tinify::new().set_key(&self.cache.api_key);

                Command::perform(
                    async move { process_images(&mut paths, tinify).await },
                    |result| match result.is_ok() {
                        true => Message::Display(LogText::Success),
                        false => Message::Display(LogText::Fail),
                    },
                )
            }
            Message::ClearPath => {
                self.cache.clear_paths();
                Command::none()
            }

            Message::ToggleTheme => {
                match self.config.theme {
                    AppTheme::Light => self.config.theme = AppTheme::Dark,
                    AppTheme::Dark => self.config.theme = AppTheme::Moonfly,
                    AppTheme::Moonfly => self.config.theme = AppTheme::Oxocarbon,
                    AppTheme::Oxocarbon => self.config.theme = AppTheme::Light,
                }
                Command::none()
            }

            Message::Display(log_text) => {
                self.cache.log_text = log_text;
                Command::none()
            }
            Message::Exit => {
                let contents = toml::to_string(&self.config).unwrap();
                Command::perform(
                    async { fs::write("./tinyrs.toml", contents).await },
                    |result| match result {
                        Ok(_) => {
                            exit(0);
                        }
                        _err => panic!("权限不足!"),
                    },
                )
            }
            Message::TurnTo(page) => match page {
                Page::Home => {
                    self.cache.page = Page::Home;
                    Command::none()
                }
                Page::Settings => {
                    self.cache.page = Page::Settings;
                    Command::none()
                }
            },
        }
    }

    fn title(&self) -> String {
        "TinyRS".to_string()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        match self.cache.page {
            Page::Home => {
                let api_input = container(
                    TextInput::new("Type APIKey Here", &self.cache.api_key)
                        .on_input(|s: String| Message::Add(Thing::APi(s)))
                        .padding(25),
                )
                .padding(60)
                .center_x();

                let log_text: &str = (&self.cache.log_text).into();
                let log_text = Text::new(log_text)
                    .style(theme::Text::Color((&self.cache.log_text).into()))
                    .size(25);
                let added_p = self.cache.rfd_opened_path.to_display();
                println!("{}", added_p);
                let added_path = row!(
                    Text::new("Added Path: ").size(22),
                    Text::new(added_p).size(20)
                );

                let basic = container(
                    row!(
                        Button::new(Text::new("AddPath").shaping(Shaping::Advanced).size(20))
                            .on_press(Message::Add(Thing::Path))
                            .style(theme::Button::custom(self.config.button_style.clone())),
                        Button::new(Text::new("ClearPath").shaping(Shaping::Advanced).size(20))
                            .on_press(Message::ClearPath)
                            .style(theme::Button::custom(self.config.button_style.clone())),
                        Button::new(Text::new("Process").shaping(Shaping::Advanced).size(20))
                            .on_press(Message::Convert)
                            .style(theme::Button::custom(self.config.button_style.clone())),
                    )
                    .spacing(8),
                )
                .padding(40)
                .center_x()
                .center_y();

                let settings = container(
                    Button::new(Text::new("ToSettings").size(25))
                        .on_press(Message::TurnTo(Page::Settings))
                        .style(theme::Button::custom(self.config.button_style.clone())),
                )
                .padding(40);
                let exit = Button::new(Text::new("Exit").size(25))
                    .on_press(Message::Exit)
                    .style(theme::Button::custom(self.config.button_style.clone()));
                column!(api_input, log_text, added_path, basic, settings, exit)
                    .spacing(9)
                    .align_items(Alignment::Center)
                    .into()
            }
            Page::Settings => column!(
                container(
                    Button::new(Text::new("ToggleTheme").shaping(Shaping::Advanced).size(20))
                        .on_press(Message::ToggleTheme)
                        .style(theme::Button::custom(self.config.button_style.clone())),
                )
                .center_x(),
                container(
                    Button::new(Text::new("ToHome").shaping(Shaping::Advanced).size(20))
                        .on_press(Message::TurnTo(Page::Home))
                        .style(theme::Button::custom(self.config.button_style.clone())),
                )
                .center_x()
            )
            .spacing(9)
            .align_items(Alignment::Center)
            .into(),
        }
    }
    fn theme(&self) -> Self::Theme {
        self.config.theme.clone().into()
    }
}
