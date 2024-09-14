// #![windows_subsystem = "windows"]

use iced::{Application, Font, Settings};
use tiny_rs::{
    state::{Cache, Config},
    App,
};
fn main() {
    App::run(Settings {
        default_font: Font::with_name("Source Han Serif SC"),
        flags: (Config::default(), Cache::default()),
        ..Default::default()
    })
    .unwrap()
}
