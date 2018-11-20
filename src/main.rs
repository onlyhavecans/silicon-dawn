extern crate rand;
extern crate shio;

#[macro_use]
extern crate horrorshow;

use horrorshow::helper::doctype;
use horrorshow::prelude::Raw;
use rand::seq::SliceRandom;
use rand::thread_rng;
use shio::context::Key;
use shio::prelude::*;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::{env, process};

const CARD_DIRECTORY: &str = "The Tarot of the Silicon Dawn";
const CARD_URI: &str = "cards";
const STANDARD_PORT: u16 = 3000;

pub struct SharedCardList;

impl Key for SharedCardList {
    type Value = Vec<String>;
}

fn main() {
    let mut port: u16 = STANDARD_PORT;

    if let Some(arg1) = env::args().nth(1) {
        if let Ok(new_port) = arg1.parse::<u16>() {
            port = new_port;
        } else {
            eprintln!("Usage: silicon-dawn <port>");
            process::exit(1);
        }
    }

    println!("Running on http://localhost:{}", port);

    Shio::default()
        .manage::<SharedCardList>(get_all_jpgs(CARD_DIRECTORY))
        .route((Method::GET, "/", show_random_card))
        .route((
            Method::GET,
            format!("/{}/{{card_name}}", CARD_URI).as_str(),
            return_card,
        )).run(format!(":{}", port))
        .unwrap();
}

fn get_all_jpgs(directory: &str) -> Vec<String> {
    let mut cards = Vec::new();
    let dir = Path::new(directory);

    println!("Caching card list, this takes a moment.");

    if dir.is_dir() {
        if let Ok(files) = fs::read_dir(dir) {
            for entry in files {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(file_type) = path.extension() {
                        if file_type == "jpg" {
                            let card_name = entry.file_name().into_string();
                            if let Ok(no_really_now_the_name) = card_name {
                                cards.push(no_really_now_the_name);
                            }
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("The expected tarot folder {:?} is missing.", dir);
        eprintln!("Shutting Down.");
        process::exit(4);
    }

    if cards.is_empty() {
        eprintln!(
            "I didn't find any of the tarot jpg files in the {:?} directory.",
            dir
        );
        eprintln!("Shutting Down.");
        process::exit(5);
    }

    println!("Successfully pulled {} cards into the cache.", cards.len());
    cards
}

fn show_random_card(ctx: Context) -> Response {
    let mut rng = thread_rng();
    let cached_cards = ctx.shared().get::<SharedCardList>();

    println!("Pulling a card on request.");

    if let Some(pick) = cached_cards.choose(&mut rng) {
        Response::with(render_card_picks(pick))
    } else {
        Response::build()
            .status(StatusCode::InternalServerError)
            .body("No rng. Hail Eris")
            .into()
    }
}

fn render_card_picks(card_name: &str) -> String {
    let card_text = &card_name.replace(".jpg", "-text.png");
    format!(
        "{}",
        html!{
            : doctype::HTML;
            html {
                head {
                    title : "Tarot of the Silicon Dawn";
                    style(TYPE="text/css") : Raw("body{background: black;color:dimgrey}");
                }
                body(bgcolor="#000000") {
                    center {
                        img(src=format!("{}/{}", CARD_URI, card_name), alt=card_name);
                        br;
                        img(src=format!("{}/{}", CARD_URI, card_text), alt=card_text);
                        br;
                        p {
                            : Raw("Everything is &copy Egypt Urnash http://egypt.urnash.com/tarot/")
                        }
                        p {
                            : Raw("Code can be found at https://onlyhavecans.works/amy/silicon-dawn")
                        }
                    }
                }
            }
        }
    )
}

fn return_card(ctx: Context) -> Response {
    let directory_path = Path::new(CARD_DIRECTORY);
    let file_name = &ctx.get::<Parameters>()["card_name"];
    let full_path = directory_path.join(file_name);

    println!("Offering up {:?} on request.", full_path);

    if let Ok(mut f) = File::open(full_path) {
        let mut buffer = Vec::new();
        if let Ok(_) = f.read_to_end(&mut buffer) {
            Response::build().body(buffer).into()
        } else {
            Response::build()
                .status(StatusCode::InternalServerError)
                .body("Unable to read file")
                .into()
        }
    } else {
        Response::build().status(StatusCode::NotFound).into()
    }
}
