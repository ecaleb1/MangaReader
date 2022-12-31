#![allow(non_snake_case)]
use iced::widget::{image, row, button};
use iced::{Element, Sandbox, Settings, Length, Alignment};

use std::{io, fs};
use std::fs::File;
use std::path::PathBuf;
use zip::{read};

pub fn main() -> iced::Result {
    read::ZipArchive::new( File::open("Comic.cbr").unwrap() ).expect("Failed to read archive").extract(".files").unwrap();
    Reader::run(Settings {id: Some(String::from("MangaReader")), window: iced::window::Settings::default(), flags: (), default_font: None, default_text_size: 16, text_multithreading: true, exit_on_close_request: true, antialiasing: true, try_opengles_first: false})
}

struct Reader {
    page: usize,
    entries: Vec<PathBuf>,
    length: usize,
}

#[derive(Debug, Clone)]
enum Message {
    NextImage,
    PreviousImage
}

impl Sandbox for Reader {
    type Message = Message;

    fn new() -> Reader {
        let mut var = fs::read_dir(".files").expect("").map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>().unwrap();
        let zip_len = var.len();
        var.sort_unstable();
        Reader {
            page: 0,
            entries: var,
            length: zip_len,
        }
    }

    fn title(&self) -> String {
        String::from("MangaReader")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NextImage => {
                if self.page+1 == self.length {
                    //Indicate last page
                } else {
                    self.page += 1;
                }
            }
            Message::PreviousImage => {
                if self.page == 0 {
                    //Indicate first page
                } else {
                    self.page -= 1;
                }
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        row![
            button(" < ").on_press(Message::PreviousImage).padding([30,10]),
            image::Viewer::new(image::Handle::from_path(&self.entries[self.page]))
            .width(Length::Fill).height(Length::Fill),
            button(" > ").on_press(Message::NextImage).padding([30,10]),
        ]
        .align_items(Alignment::Center).into()
    }
}
