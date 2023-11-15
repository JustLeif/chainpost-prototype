use std::env;

use actix_web::{get, Responder};
use askama_actix::{Template, TemplateToResponse};

#[derive(Template)]
#[template(path = "auth.html")]
struct AppAuth<'a> {
    domain: &'a str,
}

#[get("/app/auth")]
pub async fn app_auth() -> impl Responder {
    let domain = env::var("DOMAIN").expect("DOMAIN env var not set.");
    return AppAuth {
        domain: domain.as_str(),
    }
    .to_response();
}
