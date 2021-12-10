use rand::seq::SliceRandom;
use rand::thread_rng;
use std::path::Path;
use std::{fs};
use std::ffi::OsStr;

pub type CardDeck = Vec<String>;

pub struct Card {
    pub name: String,
    pub text: String,
}

pub fn get_cards(directory: &str) -> Option<CardDeck> {
    let dir = Path::new(directory);
    let files = fs::read_dir(dir).ok()?;
    let extension: &OsStr = OsStr::new("jpg");

    let cards: CardDeck = files
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .filter(|x| x.path().extension() == Some(extension) )
        .map(|x| x.file_name().into_string().unwrap())
        .collect();

    match cards.is_empty() {
        true => None,
        false => Some(cards),
    }
}

pub fn pick_a_card(cards: &CardDeck) -> Option<Card> {
    let mut rng = thread_rng();
    let pick = cards.choose(&mut rng);
    match pick {
        Some(p) => {
            Some(Card {
                name: p.to_string(),
                text: p.replace(".jpg", "-text.png"),
            })
        }
        None => None
    }
}
