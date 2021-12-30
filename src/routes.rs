use crate::cards::{pick_a_card, CardDeck};
use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;

pub async fn index(hbs: web::Data<Handlebars<'_>>, deck: web::Data<CardDeck>) -> impl Responder {
    let card = match pick_a_card(deck.get_ref()) {
        Ok(card) => card,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let data = json![{
        "card_dir": "/cards",
        "card_name": card.encoded_name(),
        "card_text": card.encoded_text(),
    }];
    let body = hbs.render("index", &data);
    match body {
        Ok(b) => HttpResponse::Ok().body(b),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn robots() -> impl Responder {
    "User-agent: *\nDisallow: /"
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
