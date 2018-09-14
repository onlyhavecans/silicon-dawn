extern crate shio;
extern crate rand;

use rand::prelude::*;
use shio::prelude::*;
use shio::context::Key;
use std::fs::{self, File};
use std::path::Path;
use std::io::Read;
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
        .route((Method::GET, format!("/{}/{{card_name}}", CARD_URI).as_str(), return_card))
        .run(format!(":{}", port))
        .unwrap();
}


fn show_random_card(ctx: Context) -> Response {
    let mut rng = thread_rng();
    let cached_cards = ctx.shared().get::<SharedCardList>();

    if let Some(pick) = rng.choose(cached_cards) {
        Response::with(render_card_picks(pick))
    } else {
        status_500()
    }
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
                            cards.push(format!("{}", entry.file_name().to_str().unwrap()));
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
        eprintln!("I didn't find any of the tarot jpg files in the {:?} directory.", dir);
        eprintln!("Shutting Down.");
        process::exit(5);
    }

    println!("Successfully pulled card list");
    cards
}


fn render_card_picks(card_name: &str) -> String {
    let card_text = card_name.replace(".jpg", "-text.png");

    format!("<html><body bgcolor=#000000><img src={uri}/{card} alt={card} /><br /><img src={uri}/{text} alt={text} /></body></html>", uri = CARD_URI, card =  card_name, text = card_text)
}


fn return_card(ctx: Context) -> Response {
    let directory_path = Path::new(CARD_DIRECTORY);
    let file_name = &ctx.get::<Parameters>()["card_name"];
    let full_path = directory_path.join(file_name);

    match File::open(full_path) {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            match f.read_to_end(&mut buffer) {
                Ok(_) => {
                    return Response::build()
                        .body(buffer)
                        .into();
                },
                Err(_) => {
                    return status_500();
                },
            };
        },
        _ => {
            return status_404();
        },
    }
}


fn status_404() -> Response {
    Response::build().status(StatusCode::NotFound).into()
}


fn status_500() -> Response {
    Response::build().status(StatusCode::InternalServerError).into()
}
