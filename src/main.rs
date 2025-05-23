//Usage: MangaReader [file]
#![allow(non_snake_case)]

use iced::{
    keyboard, Element, Length, Subscription, Theme, Color,
    alignment::{Horizontal, Vertical},
    widget::{button, column, row, text, checkbox},
    widget::svg::{self, Svg}, 
    widget::image::{self, Image},
    widget::button::{Status, Style},
    border::Border,
};

use iced_aw::widget::context_menu::ContextMenu;

use bitcode;
use serde::{Serialize, Deserialize};
use directories::ProjectDirs;

use std::{
    env, process,
    io::Read,
    fs::{File, read_dir, DirEntry, ReadDir, write, read, exists, create_dir},
};

use zip::read::ZipArchive;

const DUAL_PAGE_SVG: &'static [u8] = include_bytes!("../res/DualPage.svg");
const MANGA_MODE_SVG: &'static [u8] = include_bytes!("../res/MangaMode.svg");

pub fn main() -> iced::Result {
    iced::application("MangaReader", update, view)
        .subscription(subscription)
        .theme(theme)
        .run()
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Options {
    dual_page_mode: bool,
    manga_mode: bool,
}
impl Default for Options {
    fn default() -> Self {
        Options {
            dual_page_mode: false,
            manga_mode: false,
        }
    }
}
impl Options {
    fn save(self) {
        let dirs = ProjectDirs::from("com", "tvn3r", "MangaReader").unwrap();
        let options_file = dirs.data_dir().join("options.dat");

        if !dirs.data_dir().exists() {
            create_dir(dirs.data_dir()).expect("Unable to create data dir");
        }

        write(
            &options_file,
            bitcode::serialize(&self).unwrap()
        ).unwrap();
    }
    fn load() -> Self {
        let dirs = ProjectDirs::from("com", "tvn3r", "MangaReader").unwrap();
        let options_file = dirs.data_dir().join("options.dat");

        if exists(&options_file).unwrap() {
            println!("Loading options from file");
            bitcode::deserialize(&read(&options_file).unwrap()).unwrap()
        } else {
            println!("Creating new options");
            Options::default()
        }
    }
}

pub struct Reader {
    page: usize,
    entries: Vec<Vec<u8>>,
    length: usize,
    screen: Screen,
    options: Options,
    //dual_page_mode: bool,
    //manga_mode: bool,
}
impl Default for Reader {
    fn default() -> Reader {
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
        Reader {
            page: 0,
            entries: var,
            length: zip_len,
            screen: Screen::SinglePage,
            options: Options::load(),
            //dual_page_mode: false,
            //manga_mode: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NextImage,
    PreviousImage,
    Open,
    Close,
    FirstImage,
    LastImage,
    DualPageToggle,
    MangaModeToggle,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    SinglePage,
    DualPage,
}

fn theme(_state: &Reader) -> iced::Theme {
    Theme::Dark
}

fn update(state: &mut Reader, message: Message) {
    match message {
        Message::NextImage => {
            if state.options.manga_mode {
                if state.page > 1 {
                    state.page -= 2;
                } else if state.page > 0 {
                    state.page -= 1;
                }
            } else if state.options.dual_page_mode {
                if state.page < state.length - 2 {
                    state.page += 2;
                } else if state.page < state.length - 1 {
                    state.page += 1;
                }
            } else if state.page < state.length {
                state.page += 1;
            }
        }
        Message::PreviousImage => {
            if state.options.manga_mode {
                if state.options.dual_page_mode {
                    if state.page < state.length - 2 {
                        state.page += 2;
                    } else if state.page < state.length - 1 {
                        state.page += 1;
                    }
                } else if state.page < state.length {
                    state.page += 1;
                }
            } else if state.page > 0 {
                state.page -= 1;
            }
        }
        Message::Open => {
            //todo
        }
        Message::Close => {
            process::exit(0);
        }
        Message::FirstImage => {
            state.page = 0;
        }
        Message::LastImage => {
            if state.options.dual_page_mode {
                state.page = state.length - 1;
            } else {
                state.page = state.length;
            }
        }
        Message::DualPageToggle => {
            if state.page == state.length {
                state.page -= 1;
            }
            state.options.dual_page_mode = !state.options.dual_page_mode;
            state.options.save();
            if state.screen == Screen::DualPage {
                state.screen = Screen::SinglePage;
            } else {
                state.screen = Screen::DualPage;
            }
        }
        Message::MangaModeToggle => {
            state.options.manga_mode = !state.options.manga_mode;
            state.options.save();
        }
    }
}

fn subscription(_state: &Reader) -> Subscription<Message> {
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

fn view(state: &Reader) -> Element<Message> {
    let body: Element<Message> = match state.screen {
        Screen::SinglePage => {
            row![
                Image::new(image::Handle::from_bytes( state.entries[state.page].clone() ))
                    .width(Length::Fill).height(Length::Fill),
            ].into()
        },
        Screen::DualPage => dual_page(state),
    };

    ContextMenu::new(body, || {
        column(vec![
            //Dual Page
            button(row![
                checkbox("", state.options.dual_page_mode),
                Svg::new(svg::Handle::from_memory(DUAL_PAGE_SVG)),
                text("").width(8), //Spacer
                text("Dual Page Mode").width(130)
            ].align_y(Vertical::Center))
                .on_press(Message::DualPageToggle)
                //.style(|_t, status| custom_button(status))
                .width(220).height(35)
                .into(),
            //Manga Mode
            button(row![
                checkbox("", state.options.manga_mode),
                Svg::new(svg::Handle::from_memory(MANGA_MODE_SVG)),
                text("").width(8), //Spacer
                text("Manga Mode").width(130)
            ].align_y(Vertical::Center))
                .on_press(Message::MangaModeToggle)
                //.style(|_t, status| custom_button(status))
                .width(220).height(35)
                .into(),
        ]).into()
    }).into()
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


#[allow(unused)]
fn custom_button(status: iced::widget::button::Status) -> iced::widget::button::Style {
    match status {
        Status::Active => Style {
            background: Some(iced::Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 200.))),
            border: Border {
                color: Color::from_rgb(255., 255., 255.),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        },

        _ => Style::default(),
    }
}


//Screens
fn dual_page(state: &Reader) -> Element<Message> {
    if state.options.manga_mode {
        column![
            row![
                Image::new(image::Handle::from_bytes( state.entries[state.page+1].clone() ))
                    .height(Length::Fill),
                Image::new(image::Handle::from_bytes( state.entries[state.page].clone() ))
                    .height(Length::Fill),
            ]
        ].width(Length::Fill).align_x(Horizontal::Center).into()
    } else {
        column![
            row![
                Image::new(image::Handle::from_bytes( state.entries[state.page].clone() ))
                    .height(Length::Fill),
                Image::new(image::Handle::from_bytes( state.entries[state.page+1].clone() ))
                    .height(Length::Fill),
            ]
        ].width(Length::Fill).align_x(Horizontal::Center).into()
    }
}
/*
fn long_strip(state: &Reader) -> Element<Message> {
    column![
        Image::new(image::Handle::from_bytes( state.entries[state.page].clone() ))
            .width(Length::Fill)
    ].into()
}
*/
