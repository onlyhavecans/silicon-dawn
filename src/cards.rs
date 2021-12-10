use rand::seq::SliceRandom;
use rand::thread_rng;
use std::path::Path;
use std::{fs, io};
use std::ffi::OsStr;

pub type CardDeck = Vec<String>;

pub struct Card {
    pub name: String,
    pub text: String,
}

pub fn get_cards(directory: &str) -> Result<CardDeck, io::Error> {
    let dir = Path::new(directory);
    let files = fs::read_dir(dir)?;
    let extension: &OsStr = OsStr::new("jpg");

    println!("Caching card list, this takes a moment.");
    let cards: CardDeck = files
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .filter(|x| x.path().extension() == Some(extension) )
        .map(|x| x.file_name().into_string().unwrap())
        .collect();

    println!("Successfully pulled {} cards into the cache.", cards.len());
    Ok(cards)
}

pub fn pick_a_card(cards: &CardDeck) -> Result<Card, String> {
    let mut rng = thread_rng();
    let pick = cards.choose(&mut rng);
    match pick {
        Some(p) => {
            let text = p.replace(".jpg", "-text.png");
            Ok(Card {
                name: p.to_string(),
                text: text,
            })
        }
        None => Err("Nothing Picked!".to_string()),
    }
}
