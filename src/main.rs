#![feature(proc_macro_hygiene, decl_macro)]

use rand::seq::SliceRandom;
use rand::thread_rng;
use rocket::http::Status;
use rocket::{get, routes, State};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process;

const CARD_DIRECTORY: &str = "The-Tarot-of-the-Silicon-Dawn";
const CARD_URI: &str = "/cards";

struct SharedCardList {
    cards: Vec<String>,
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

#[get("/")]
fn draw_card(state: State<SharedCardList>) -> Result<Template, Status> {
    let mut rng = thread_rng();

    if let Some(pick) = state.cards.choose(&mut rng) {
        let card_text = &pick.replace(".jpg", "-text.png");
        let mut context = HashMap::new();
        context.insert("card_dir", CARD_URI);
        context.insert("card_name", pick);
        context.insert("card_text", card_text);

        Ok(Template::render("index", &context))
    } else {
        Err(Status::InternalServerError)
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![draw_card])
        .mount(CARD_URI, StaticFiles::from(CARD_DIRECTORY))
        .attach(Template::fairing())
}

fn main() {
    let config = SharedCardList {
        cards: get_all_jpgs(CARD_DIRECTORY),
    };

    rocket().manage(config).launch();
}
