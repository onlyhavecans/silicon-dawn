extern crate shio;
extern crate rand;

use rand::prelude::*;
use shio::prelude::*;
use std::fs::{self, File};
use std::path::Path;
use std::io::Read;
use std::{env, process};

const CARD_DIRECTORY: &str = "The Tarot of the Silicon Dawn";
const CARD_URI: &str = "cards";
const STANDARD_PORT: u16 = 3000;


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
    Shio::default()
        .route((Method::GET, "/", pick_card))
        .route((Method::GET, format!("/{}/{{card_name}}", CARD_URI).as_str(), return_card))
        .run(format!(":{}", port))
        .unwrap();
}


fn pick_card(_: Context) -> Response {
    if let Ok(cards) = list_all_jpgs(Path::new(CARD_DIRECTORY)) {

        let mut rng = thread_rng();
        let pick = rng.choose(&cards).unwrap();

        Response::with(render_card_picks(pick))
    } else {
        status_500()
    }
}


fn list_all_jpgs(dir: &Path) -> Result<Vec<String>, &str> {
    let mut cards = Vec::new();

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
        return Err("Not a directory")
    }

    Ok(cards)
}


fn render_card_picks(card_name: &str) -> String {
    let card_base = card_name.replace(".jpg","");
    let card_text = format!("{}-text.png", card_base);

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
