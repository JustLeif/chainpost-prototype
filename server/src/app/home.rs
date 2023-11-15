use crate::services::{check_auth::check_auth, get_uid::get_uid_from_jwt};
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateToResponse};

#[derive(Template)]
#[template(path = "home.html")]
struct AppHome {
    uid: String,
    display: String,
}

#[get("/app/home")]
pub async fn app_home(req: HttpRequest) -> impl Responder {
    if check_auth(&req) == false {
        return HttpResponse::Found()
            .append_header(("Location", "/app/auth"))
            .finish();
    } else {
        if let Ok(uid) = get_uid_from_jwt(
            req.cookie("chainpost_jwt")
                .expect("JWT WAS NOT SET AFTER CHECKING WITH CHECK_AUTH!")
                .value(),
        ) {
            return AppHome {
                display: format!("{}...", &uid[0..15]),
                uid: uid,
            }
            .to_response();
        } else {
            return HttpResponse::InternalServerError().finish();
        }
    }
}
