mod api_response;
mod app;
mod config;
mod url;

use crate::{
    app::{create, get, index, oops},
    config::CONFIG,
};
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis = redis::Client::open(format!("redis://{}:{}", CONFIG.urs.host, CONFIG.redis.port)).unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis.clone()))
            .wrap(Cors::default().allowed_origin(&CONFIG.cors.server).allowed_origin(&CONFIG.cors.client))
            .wrap(Logger::default())
            .default_service(web::route().to(oops))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/create").route(web::post().to(create)))
            .service(web::resource("/{id}").route(web::get().to(get)))
    })
    .bind(format!("{}:{}", CONFIG.urs.host, CONFIG.urs.port))?
    .run()
    .await
}
