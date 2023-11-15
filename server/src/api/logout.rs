use actix_web::cookie::Cookie;
use actix_web::{get, HttpResponse, Responder};

#[get("/api/logout")]
pub async fn api_logout() -> impl Responder {
    let cookie = Cookie::build("chainpost_jwt", "")
        .path("/") // Set the path if necessary; adjust as needed
        .http_only(true) // Ensure it's still HTTP-only
        .finish();

    return HttpResponse::Ok()
        .cookie(cookie)
        .append_header(("HX-Redirect", "/app/auth"))
        .finish();
}
