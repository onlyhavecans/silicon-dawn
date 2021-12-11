use rand::seq::SliceRandom;
use rand::thread_rng;
use std::ffi::OsStr;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;

pub type CardDeck = Vec<String>;

#[derive(PartialEq, Eq, Debug)]
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

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn card_default() {
        let c = Card::new("fool.jpg");
        assert_eq!("fool.jpg", c.encoded_name());
        assert_eq!("fool-text.png", c.encoded_text());
    }

    #[test]
    fn card_encode() {
        let c = Card::new("MA-8½-Maya.jpg");
        assert_eq!("MA-8%C2%BD-Maya.jpg", c.encoded_name());
        assert_eq!("MA-8%C2%BD-Maya-text.png", c.encoded_text());
    }

    #[test]
    fn card_bad() {
        let c = Card::new("bad");
        assert_eq!("bad", c.encoded_name());
        assert_eq!("bad", c.encoded_text());
    }
}

pub fn get_cards(directory: &str) -> Option<CardDeck> {
    let dir = Path::new(directory);
    let files = fs::read_dir(dir).ok()?;

    get_cards_from_dir(files)
}

pub fn pick_a_card(cards: &[String]) -> Option<Card> {
    let mut rng = thread_rng();
    let pick = cards.choose(&mut rng);
    pick.map(|p| Card::new(p))
}

#[cfg(test)]
mod pick_tests {
    use super::*;

    #[test]
    fn pick_single_card() {
        let d: Vec<String> = vec!["fool.jpg".to_string()];
        let c = Card::new("fool.jpg");
        let r = pick_a_card(&d);
        assert_eq!(Some(c), r);
    }

    #[test]
    fn pick_no_card() {
        let d: Vec<String> = Vec::new();
        let r = pick_a_card(&d);
        assert_eq!(None, r)
    }
}

fn get_cards_from_dir(files: ReadDir) -> Option<CardDeck> {
    let extension: &OsStr = OsStr::new("jpg");

    let cards: CardDeck = files
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .filter(|x| x.path().extension() == Some(extension))
        .map(|x| x.file_name().into_string().unwrap())
        .collect();

    match cards.is_empty() {
        true => None,
        false => Some(cards),
    }
}

#[cfg(test)]
mod get_tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    // #[test]
    // fn get_cards() {
    //     let dir = Path::new("./tests");
    //     let files = fs::read_dir(dir).unwrap();
    //     let c = get_cards_from_dir(files);
    //     let t: Vec<String> = vec!["test.jpg".to_string()];
    //     assert_eq!(Some(t), c);
    // }

    #[test]
    fn get_no_cards() {
        let dir = Path::new(".");
        let files = fs::read_dir(dir).unwrap();
        let c = get_cards_from_dir(files);
        assert_eq!(None, c);
    }
}