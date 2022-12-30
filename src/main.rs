#![allow(non_snake_case)]
use iced::widget::{column, container, image, row, button};
use iced::{Element, Sandbox, Settings, Length};

use std::{fs, io};
use std::path::PathBuf;


pub fn main() -> iced::Result {
    Reader::run(Settings {id: Some(String::from("MangaReader")), window: iced::window::Settings::default(), flags: (), default_font: None, default_text_size: 16, text_multithreading: true, exit_on_close_request: true, antialiasing: true, try_opengles_first: false})
}

struct Reader {
    i: usize,
    entries: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
enum Message {
    NextImage,
    PreviousImage
}

impl Sandbox for Reader {
    type Message = Message;

    fn new() -> Reader {
        Reader {
            entries: fs::read_dir("./img").expect("").map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>().unwrap(),
            i: 0,
        }
    }

    fn title(&self) -> String {
        String::from("MangaReader")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NextImage => {
                self.i += 1;
            }
            Message::PreviousImage => {
                self.i -= 1;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        row![
            button("Previous").on_press(Message::PreviousImage),
            image::Image::new(image::Handle::from_path(&self.entries[self.i]))
            .width(Length::Fill).height(Length::Fill),
            button("Next").on_press(Message::NextImage),
        ]
        .into()
        //container::Container::new(image)
    }
}