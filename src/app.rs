use crate::api_response::{data::ApiDataResponse, ok::ApiResponse};
use actix_web::{
    error,
    http::StatusCode,
    web::{self, Redirect},
    HttpResponse, Responder,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub fn generate_id() -> String {
    (0..=6).map(|_| thread_rng().sample(Alphanumeric) as char).collect()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Url {
    pub id: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UrlNew {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UrlId {
    pub id: String,
}

impl Url {
    pub fn new(id: &str, url: &str) -> Self {
        Self {
            id: id.to_owned(),
            url: url.to_owned(),
        }
    }
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::set().status(StatusCode::OK).message("urs").build())
}

pub async fn oops() -> HttpResponse {
    HttpResponse::NotFound().json(ApiResponse::set().status(StatusCode::NOT_FOUND).message("Not found").build())
}

pub async fn create(redis: web::Data<redis::Client>, data: web::Query<UrlNew>) -> actix_web::Result<impl Responder> {
    let mut conn = redis.get_connection_manager().await.map_err(error::ErrorInternalServerError).unwrap();

    let url = Url::new(&generate_id(), &data.url);

    let check = redis::Cmd::set(url.id, url.url)
        .query_async::<_, String>(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError);

    match check {
        Ok(url) => Ok(HttpResponse::Created()
            .json(ApiDataResponse::set(url).status(StatusCode::CREATED).message("Successfully shortened url").build())),
        _ => Ok(HttpResponse::InternalServerError()
            .json(ApiResponse::set().status(StatusCode::CREATED).message("Failed to shortened url").build())),
    }
}

pub async fn get(redis: web::Data<redis::Client>, data: web::Path<UrlId>) -> actix_web::Result<impl Responder> {
    let mut conn = redis.get_connection_manager().await.map_err(error::ErrorInternalServerError).unwrap();

    let check =
        redis::Cmd::get(&data.id).query_async::<_, String>(&mut conn).await.map_err(error::ErrorInternalServerError);

    match check {
        Ok(url) => {
            log::info!("Redirecting to: {}", url);
            Ok(Redirect::to(url))
        },
        Err(_) => Ok(Redirect::to("/404")),
    }
}
