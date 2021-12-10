use crate::cards::CardDeck;
use crate::routes::{index, robots};
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use std::net::TcpListener;

pub fn run(listener: TcpListener, deck: CardDeck) -> Result<Server, std::io::Error> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let deck_ref = web::Data::new(deck);
    let server = HttpServer::new(move || {
        App::new()
            .service(Files::new("/cards", "./cards"))
            .route("/robots.txt", web::get().to(robots))
            .route("/", web::get().to(index))
            .app_data(deck_ref.clone())
            .app_data(handlebars_ref.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
