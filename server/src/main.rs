use actix_cors::Cors;
use actix_web::{App, HttpServer};

pub mod api;
pub mod app;
pub mod db;
pub mod services;
use api::{
    auth::api_auth_cardano,
    logout::api_logout,
    post::{api_delete_post, api_get_post, api_post, api_post_csv},
};
use app::{
    auth::app_auth, home::app_home, post::app_post, profile::app_profile, search::app_search,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(api_auth_cardano)
            .service(api_post)
            .service(app_auth)
            .service(app_home)
            .service(app_post)
            .service(api_get_post)
            .service(api_delete_post)
            .service(api_post_csv)
            .service(api_logout)
            .service(app_profile)
            .service(app_search)
            .service(redirect_to_app_home)
            .service(actix_files::Files::new("static/js", "./static/js"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

use actix_web::{get, Responder, HttpResponse, http};
#[get("/")]
async fn redirect_to_app_home() -> impl Responder {
    HttpResponse::Found()
        .header(http::header::LOCATION, "/app/home")
        .finish()
}