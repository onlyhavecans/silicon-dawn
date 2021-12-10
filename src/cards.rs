use rand::seq::SliceRandom;
use rand::thread_rng;
use std::path::Path;
use std::{fs, io};

pub type CardDeck = Vec<String>;

pub struct Card {
    pub name: String,
    pub text: String,
}

pub fn get_cards(directory: &str) -> Result<CardDeck, io::Error> {
    let mut cards: CardDeck = Vec::new();
    let dir = Path::new(directory);

    println!("Caching card list, this takes a moment.");

    let files = fs::read_dir(dir)?;

    for entry in files {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(file_type) = path.extension() {
                if file_type == "jpg" {
                    let card_name = entry.file_name().into_string();
                    if let Ok(name) = card_name {
                        cards.push(name);
                    }
                }
            }
        }
    }

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
