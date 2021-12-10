use crate::cards::{pick_a_card, CardDeck};
use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;

pub async fn index(hbs: web::Data<Handlebars<'_>>, deck: web::Data<CardDeck>) -> impl Responder {
    let card = pick_a_card(deck.get_ref()).unwrap();
    let data = json![{
        "card_dir": "/cards",
        "card_name": card.name,
        "card_text": card.text,
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
