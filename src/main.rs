//Usage: MangaReader [file]
#![allow(non_snake_case)]

use iced::{Settings, Application};

mod gui;

pub fn main() -> iced::Result {
    gui::Reader::run(Settings {
        id: Some(String::from("MangaReader")), 
        window: iced::window::Settings::default(), 
        flags: (),
        default_font: None, 
        default_text_size: 16, 
        text_multithreading: true, 
        exit_on_close_request: true, 
        antialiasing: true, 
        try_opengles_first: false
    })
}
