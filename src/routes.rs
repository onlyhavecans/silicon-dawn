use crate::cards::{pick_a_card, CardDeck};
use actix_web::{web, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    card_dir: &'a str,
    card_name: &'a str,
    card_text: &'a str,
}

/// Draw and return cards on every index page load
pub async fn index(deck: web::Data<CardDeck>) -> impl Responder {
    let card = match pick_a_card(deck.get_ref()) {
        Some(card) => card,
        None => return HttpResponse::InternalServerError().finish(),
    };

    let card_name = card.encoded_name();
    let card_text = card.encoded_text();

    let template = IndexTemplate {
        card_dir: "/cards",
        card_name: card_name.as_str(),
        card_text: card_text.as_str(),
    };

    match template.render() {
        Ok(b) => HttpResponse::Ok().body(b),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Hardcoded robots to discourage those who still respect it
pub async fn robots() -> impl Responder {
    "User-agent: *\nDisallow: /"
}

/// Very basic healthcheck for being in a container or test
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
