use actix_web::{get, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateToResponse};

use crate::services::check_auth::check_auth;

#[derive(Template)]
#[template(path = "post.html")]
struct AppPost {}

#[get("/app/post")]
pub async fn app_post(req: HttpRequest) -> impl Responder {
    if check_auth(&req) == false {
        return HttpResponse::Found()
            .append_header(("Location", "/app/auth"))
            .finish();
    } else {
        return AppPost {}.to_response();
    }
}
