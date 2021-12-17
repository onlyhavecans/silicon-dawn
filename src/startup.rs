use crate::cards::CardDeck;
use crate::routes::{health_check, index, robots};
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    deck: CardDeck,
    serve_from: &str,
) -> Result<Server, std::io::Error> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let deck_ref = web::Data::new(deck);

    let deck_path = format!("./{}", serve_from);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(Files::new("/cards", deck_path.as_str()))
            .route("/healthcheck", web::get().to(health_check))
            .route("/robots.txt", web::get().to(robots))
            .route("/", web::get().to(index))
            .app_data(deck_ref.clone())
            .app_data(handlebars_ref.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
