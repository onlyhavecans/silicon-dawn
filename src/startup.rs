use crate::cards::CardDeck;
use crate::routes::{health_check, index, robots};
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    deck: CardDeck,
    serve_from: &str,
) -> Result<Server, std::io::Error> {
    let deck_ref = web::Data::new(deck);

    let deck_path = format!("./{}", serve_from);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(Files::new("/cards", deck_path.as_str()))
            .route("/health_check", web::get().to(health_check))
            .route("/robots.txt", web::get().to(robots))
            .route("/", web::get().to(index))
            .app_data(deck_ref.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
