use actix_web::{get, web, Responder};
use askama_actix::{Template, TemplateToResponse};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "profile.html")]
struct AppProfile {
    uid: String,
}

#[derive(Deserialize)]
pub struct AppProfileParams {
    uid: String,
}

#[get("/app/profile")]
pub async fn app_profile(params: web::Query<AppProfileParams>) -> impl Responder {
    return AppProfile {
        uid: params.uid.clone(),
    }
    .to_response();
}
