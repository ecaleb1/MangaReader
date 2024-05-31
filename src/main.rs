//Usage: MangaReader [file]
#![allow(non_snake_case)]

use iced::{
    Length, Alignment, Application, Command, Settings, Subscription, Theme, Element,
    widget::{row, image::{Image, Handle}},
    executor, keyboard,
    window::Id,
};

use std::{env, process};
use std::io::Read;
use std::fs::{File, read_dir, DirEntry, ReadDir};

use zip::read::ZipArchive;

pub fn main() -> iced::Result {
        Reader::run(Settings {
        id: Some(String::from("MangaReader")), 
        window: iced::window::Settings::default(), 
        flags: (),
        //default_font: None, 
        //default_text_size: 16., 
        antialiasing: true, 
        ..Default::default()
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
    Close,
    FirstImage,
    LastImage,
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
            process::exit(1);
        }
        
        let mut var: Vec<Vec<u8>> = Vec::new();
        let f = File::open(&args[1]).expect("Failed to read File");

        if f.metadata().unwrap().is_dir() {
            var = sort_to_vec(read_dir(&args[1]).unwrap());
        }
        else {
            //Open zip archive
            let mut archive = ZipArchive::new(f).unwrap();

            //Load file names into vec & sort
            let shit = ZipArchive::new(File::open(&args[1]).unwrap()).unwrap();
            let mut names: Vec<&str> = shit.file_names().collect();
            names.sort_unstable();
            //dbg!(&names);

            //Load image bytes into Vec ordered using sorted names
            for i in 0..archive.len() {
                if archive.by_name( names[i] ).unwrap().is_file() {
                    let mut x: Vec<u8> = Vec::new();
                    let _ = &archive.by_name( names[i] ).unwrap().read_to_end( &mut x );
                    var.push(x);
                    //dbg!( &names[i] );
                }
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

    fn theme(&self) -> iced::Theme {
        Theme::Dark
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
            Message::Close => {
                iced::window::close(Id::MAIN)
            }
            Message::FirstImage => {
                self.page = 0;
                Command::none()
            }
            Message::LastImage => {
                self.page = self.length;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        fn handle_hotkey(
            key: keyboard::Key,
            _modifiers: keyboard::Modifiers,
        ) -> Option<Message> {
            use keyboard::key;

            match key.as_ref() {
                keyboard::Key::Named(key::Named::Escape) => {
                    Some(Message::Close)
                }
                keyboard::Key::Character("q") => Some(Message::Close),
                keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::NextImage),
                keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::PreviousImage),
                keyboard::Key::Named(key::Named::Home) => Some(Message::FirstImage),
                keyboard::Key::Named(key::Named::End) => Some(Message::LastImage),
                _ => None,
            }
        }
        Subscription::batch(vec![keyboard::on_key_press(handle_hotkey)])
    }

    fn view(&self) -> Element<Self::Message> {
        row![
            Image::new(
                Handle::from_memory( self.entries[self.page].clone() ))
            .width(Length::Fill).height(Length::Fill),
        ].align_items(Alignment::Center).into()
    }
}


fn sort_to_vec(dir: ReadDir) -> Vec<Vec<u8>> {
    let mut data: Vec<DirEntry> = dir.map(|x| x.unwrap()).collect();
    let mut out: Vec<Vec<u8>> = Vec::new();

    let n = data.len();
    let mut i = 0;
    while i < n-1 {
        let mut min_index = i;
        let mut j = i + 1;

        while j < n {
            if data[j].file_name() < data[min_index].file_name() {
                min_index = j;
            }
            j+=1;
        }
        //dbg!(&data[min_index]);
        data.swap(i, min_index);
        i+=1;
    }

    //Read each file and push to byte vector
    for i in 0..data.len()-1 {
        let mut buf: Vec<u8> = Vec::new();
        File::open(data[i].path()).unwrap().read_to_end(&mut buf).expect("Failed to read File");
        out.push(buf);
    }
    return out;
}
