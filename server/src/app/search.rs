use actix_web::{get, Responder};
use askama_actix::{Template, TemplateToResponse};

#[derive(Template)]
#[template(path = "search.html")]
struct AppSearch {}

#[get("/app/search")]
pub async fn app_search() -> impl Responder {
    return AppSearch {}.to_response();
}
