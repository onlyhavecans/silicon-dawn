use silicon_dawn::cards::get_cards;
use silicon_dawn::configuration::get_configuration;
use silicon_dawn::startup::run;
use std::net::TcpListener;

const CARD_DIRECTORY: &str = "cards";

// #[get("/")]
// fn draw_card(state: State<SharedCardList>) -> Result<Template, Status> {
//     let mut rng = thread_rng();
//
//     if let Some(pick) = state.cards.choose(&mut rng) {
//         let card_text = &pick.replace(".jpg", "-text.png");
//         let mut context = HashMap::new();
//         context.insert("card_dir", CARD_URI);
//         context.insert("card_name", pick);
//         context.insert("card_text", card_text);
//         Ok(Template::render("index", &context))
//     } else {
//         Err(Status::InternalServerError)
//     }
// }

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
