//Usage: MangaReader [file]
#![allow(non_snake_case)]

use iced::widget::{image, row, button};
use iced::{Length, Alignment, Application, Command, Subscription, Settings};
use iced::executor;
use iced::keyboard;
use iced_native::Event;

use std::{env, process};
use std::io::Read;
use std::fs::File;
use zip::read::ZipArchive;
//use xdg::BaseDirectories;
//use directories::ProjectDirs;

//mod image_c;
mod viewer;
use viewer::Viewer;

use iced::Theme;
//mod theme;
//use crate::theme::Theme;

use iced::Element;
//mod widget;
//use crate::widget::Element;

pub fn main() -> iced::Result {
        Reader::run(Settings {
        id: Some(String::from("MangaReader")), 
        window: iced::window::Settings::default(), 
        flags: (),
        default_font: None, 
        default_text_size: 16., 
        text_multithreading: true, 
        exit_on_close_request: true, 
        antialiasing: true, 
        try_opengles_first: false
    })
}

pub struct Reader {
    page: usize,
    entries: Vec<Vec<u8>>,
    length: usize,
    //zoom: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    NextImage,
    PreviousImage,
    Open,
    EventOccurred(iced_native::Event),
    //Zoom(i32),
}

impl Application for Reader {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Reader, Command<Message>) {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            println!("Usage: MangaReader [file]");
            process::exit(0);
        }
        
        //Read files into Vec
        let mut archive = ZipArchive::new(
            File::open(&args[1]).expect("Failed to read file")
            ).unwrap();

        let shit = ZipArchive::new(File::open(&args[1]).unwrap()).unwrap();
        let mut names: Vec<&str> = shit.file_names().collect();
        names.sort_unstable();
        //dbg!(&names);
            
        let mut var: Vec<Vec<u8>> = Vec::new();
        for i in 0..archive.len() {
            if archive.by_name( names[i] ).unwrap().is_file() {
                let mut x: Vec<u8> = Vec::new();
                let _ = &archive.by_name( names[i] ).unwrap().read_to_end( &mut x );
                var.push(x);
                dbg!( &names[i] );
            }
        }

        //Create GUI
        let zip_len = var.len() - 1;
        (Reader {
            page: 0,
            entries: var,
            length: zip_len,
            //zoom: 1.0,
        },
        Command::none())
    }

    fn title(&self) -> String {
        String::from("MangaReader")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::NextImage => {
                if self.page+1 == self.length {
                    //Indicate last page
                    Command::none()
                } else {
                    self.page += 1;
                    Command::none()
                }
            }
            Message::PreviousImage => {
                if self.page == 0 {
                    //Indicate first page
                    Command::none()
                } else {
                    self.page -= 1;
                    Command::none()
                }
            }
            Message::Open => {
                Command::none()
            }
            /*
            Message::Zoom(x) => {
                self.zoom = match x {
                    25 => 0.25,
                    50 => 0.5,
                    100 => 1.0,
                    125 => 1.25,
                    150 => 1.5,
                    175 => 1.75,
                    200 => 2.0,
                };
                Command::none()
            }
            */

            // Keyboard Input
            Message::EventOccurred(event) => {
                match event {
                    Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                        match key_code {
                            keyboard::KeyCode::Right => {
                                if self.page == self.length {
                                    //Indicate first page
                                    Command::none()
                                } else {
                                    self.page += 1; 
                                    Command::none()
                                }
                            },
                            keyboard::KeyCode::Left => {
                                if self.page == 0 {
                                    //Indicate first page
                                    Command::none()
                                } else {
                                    self.page -= 1;
                                    Command::none()
                                }
                            },
                            keyboard::KeyCode::Home => {
                                self.page = 0;
                                Command::none()
                            },
                            keyboard::KeyCode::End => {
                                self.page = self.length;
                                Command::none()
                            },
                            _ => Command::none(),
                        }
                    },
                    _ => Command::none(),
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Self::Message> {
        row![
            button(" < ").on_press(Message::PreviousImage).padding([30,10]),

            Viewer::new(
            //image::Image::new(
                image::Handle::from_memory( self.entries[self.page].clone() ))
            .width(Length::Fill).height(Length::Fill),

            button(" > ").on_press(Message::NextImage).padding([30,10]),
        ].align_items(Alignment::Center).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
