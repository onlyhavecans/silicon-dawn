use rand::seq::SliceRandom;
use rand::thread_rng;
use std::ffi::OsStr;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;

pub type CardDeck = Vec<String>;

pub struct Card {
    name: String,
    text: String,
}

impl Card {
    pub fn new(file_name: &str) -> Self {
        Card {
            name: file_name.to_string(),
            text: file_name.replace(".jpg", "-text.png"),
        }
    }

    pub fn encoded_name(&self) -> String {
        urlencoding::encode(self.name.as_str()).to_string()
    }

    pub fn encoded_text(&self) -> String {
        urlencoding::encode(self.text.as_str()).to_string()
    }
}

pub fn get_cards(directory: &str) -> Option<CardDeck> {
    let dir = Path::new(directory);
    let files = fs::read_dir(dir).ok()?;

    let cards = get_cards_from_dir(files);
    match cards.is_empty() {
        true => None,
        false => Some(cards),
    }
}

pub fn pick_a_card(cards: &[String]) -> Option<Card> {
    let mut rng = thread_rng();
    let pick = cards.choose(&mut rng);
    pick.map(|p| Card::new(p))
}

fn get_cards_from_dir(files: ReadDir) -> CardDeck {
    let extension: &OsStr = OsStr::new("jpg");

    files
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .filter(|x| x.path().extension() == Some(extension))
        .map(|x| x.file_name().into_string().unwrap())
        .collect()
}
