use std::fs;

// #![windows_subsystem = "windows"]
use iced::{Application, Font, Settings};
use tiny_rs::{
    state::{Cache, Config},
    App,
};

// static FONT: &[u8] = include_bytes!("../MiSans-Normal.ttf");

fn main() {
    // let settings = Settings {
    //     default_font: Font::with_name("MiSans"),
    //     ..Default::default()
    // };

    // 读取配置文件

    let config = match fs::read_to_string("./tinyrs.toml") {
        Ok(config_str) => {
            toml::from_str::<Config>(&config_str).expect("Failed to parse the config file.")
        }
        Err(_) => {
            println!("No config file found, using default settings.");
            Config::default()
        }
    };

    App::run(Settings {
        default_font: Font::with_name("MiSans"),
        flags: (config, Cache::default()),
        ..Default::default()
    })
    .unwrap()
}
