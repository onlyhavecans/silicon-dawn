use silicon_dawn::cards::get_cards;
use silicon_dawn::configuration::get_configuration;
use silicon_dawn::startup::run;
use std::net::TcpListener;

const CARD_DIRECTORY: &str = "cards";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cards = get_cards(CARD_DIRECTORY)?;
    if cards.is_empty() {
        panic!("No cards loaded from {}", CARD_DIRECTORY);
    };
    println!("Pulled {} cards from {}", cards.len(), CARD_DIRECTORY);

    let configuration = get_configuration().expect("failed to read config.");

    let listener = TcpListener::bind((
        configuration.application_address,
        configuration.application_port,
    ))
    .expect("failed to bind to port");
    run(listener, cards)?.await
}
