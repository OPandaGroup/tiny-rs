use iced::widget::text::Shaping;
use iced::widget::{column, container, row, Button, Text, TextInput};
use iced::{executor, theme, Alignment, Application, Command};
use message::Thing;
use state::app_theme::AppTheme;
use state::log_text_state::LogText;
use state::process_images;
use tinify::async_bin::Tinify;

use self::message::Message;

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
        }
    }

    fn title(&self) -> String {
        "TinyRS".to_string()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
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
            column!(
                Button::new(Text::new("ToggleTheme").shaping(Shaping::Advanced).size(20))
                    .on_press(Message::ToggleTheme)
                    .style(theme::Button::custom(self.config.button_style.clone())),
            )
            .align_items(Alignment::Center)
            .spacing(10),
        )
        .padding(40);

        column!(api_input, log_text, added_path, basic, settings)
            .spacing(9)
            .align_items(Alignment::Center)
            .into()
    }
    fn theme(&self) -> Self::Theme {
        self.config.theme.clone().into()
    }
}
