use log::info;
use silicon_dawn::cards::get_cards;
use silicon_dawn::configuration::Settings;
use silicon_dawn::startup::run;
use std::net::TcpListener;

const DECK_DIRECTORY: &str = "cards";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Caching card list");
    let cards = get_cards(DECK_DIRECTORY).expect("No cards loaded from card directory");
    info!("Pulled {} cards from {}", cards.len(), DECK_DIRECTORY);

    let configuration = Settings::new().expect("Failed to read config.");

    let listener = TcpListener::bind((
        configuration.application_address,
        configuration.application_port,
    ))
    .expect("Failed to bind to port");

    run(listener, cards, DECK_DIRECTORY)?.await
}
