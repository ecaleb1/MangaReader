//Usage: MangaReader [file]
#![allow(non_snake_case)]
use iced::widget::{image, row, button, column};
use iced::{Element, Settings, Length, Alignment, Application, Command, Theme, Subscription};
use iced::executor;
use iced::keyboard;
use iced_native::Event;

use std::{io, env, process};
use std::fs::{File, remove_file, read_dir, remove_dir_all, create_dir};
use std::path::PathBuf;
use zip::read;
//use xdg::BaseDirectories;
use directories::ProjectDirs;


pub fn main() -> iced::Result {
    Reader::run(Settings {
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

struct Reader {
    page: usize,
    entries: Vec<PathBuf>,
    length: usize,
}

#[derive(Debug, Clone)]
enum Message {
    NextImage,
    PreviousImage,
    EventOccurred(iced_native::Event),
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
        
        //Define Data Directory
        let file_path = ProjectDirs::from("", "", "MangaReader").unwrap();
        match &file_path.project_path().try_exists() {
            Ok(true)  => {},
            Ok(false) => {
                println!("Creating: {}", &file_path.project_path().display());
            },
            Err(why)  => { panic!("{}", why); },
        }
        match &file_path.cache_dir().try_exists() {
            Ok(true)  => {},
            Ok(false) => { 
                println!("Creating: {}", &file_path.cache_dir().display());
                create_dir(&file_path.cache_dir()); 
            },
            Err(why)  => { panic!("{}", why); },
        }

        //Clear Directory
        for file in read_dir(&file_path.cache_dir()).unwrap() {
            let path = file.unwrap().path();
            match remove_file(&path) {
                Ok(_)  => {},
                Err(_) => {remove_dir_all(&path);},
            }
        };
        
        //Extract Comic from Archive
        read::ZipArchive::new( 
            File::open(&args[1]).expect("Couldn't find file") )
            .expect("Failed to read archive")
            .extract(&file_path.cache_dir()).unwrap();
        
        //Read files into Vec
        let mut dir = read_dir(&file_path.cache_dir()).unwrap();
        let mut var: Vec<PathBuf>;
        //Check if there is an extracted folder in .files directory
        let x = dir.nth(0).unwrap().unwrap();
        if x.file_type().unwrap().is_dir() {
            var = read_dir(x.path()).unwrap()
                .map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>().unwrap();
        } else {
            var = read_dir(&file_path.cache_dir()).unwrap()
                .map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>().unwrap();
        }

        //Create GUI
        let zip_len = var.len() - 1;
        var.sort_unstable();
        (Reader {
            page: 0,
            entries: var,
            length: zip_len,
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
        column![
        //Top bar

        //Main Body
        row![
            button(" < ").on_press(Message::PreviousImage).padding([30,10]),
            image::Viewer::new(image::Handle::from_path(&self.entries[self.page]))
            .width(Length::Fill).height(Length::Fill),
            button(" > ").on_press(Message::NextImage).padding([30,10]),
        ].align_items(Alignment::Center)
        ].align_items(Alignment::Center).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
