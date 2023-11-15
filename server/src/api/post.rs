use actix_web::web::Form;
use actix_web::{delete, get, post, HttpRequest, HttpResponse, Responder};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::db::init_db;
use crate::db::posts::{create_post, delete_post, get_posts_by_uid};
use crate::services::check_auth::check_auth;
use crate::services::get_uid::get_uid_from_jwt;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PostForm {
    post_text: String,
}

#[post("/api/post")]
async fn api_post(req: HttpRequest, form: Form<PostForm>) -> impl Responder {
    if form.post_text.trim().is_empty() {
        // Return an HTML response with an error message
        debug!("Form text is empty!");
        return HttpResponse::Ok().body("<p class='error'>Post cannot be empty!</p>");
    } else {
        if form.post_text.len() > 300 {
            return HttpResponse::Ok()
                .body("<p class='success'> Post text is greater than 300!</p>");
        }
        if check_auth(&req) {
            let jwt = req
                .cookie("chainpost_jwt")
                .expect("Failed to find chainpost_jwt after check_auth validation!");
            let uid = get_uid_from_jwt(jwt.value()).expect("Could not get the uid from JWT!");
            let client = init_db().await;
            if let Ok(_) = create_post(&client, form.post_text.clone(), uid).await {
                return HttpResponse::Created()
                    .body("<p class='success'> Post created successfully!</p>");
            } else {
                return HttpResponse::Ok()
                    .body("<p class='success'> Error when creating post, try again!</p>");
            }
        } else {
            return HttpResponse::Unauthorized().finish();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};
    
    #[actix_web::test]
    async fn test_api_post() {
        let app = test::init_service(App::new().service(api_post)).await;
        let req = test::TestRequest::post().uri("/api/post").set_form(PostForm { post_text: String::from("Fake Post")}).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED, "Request without JWT was not denied!");
    }
}

use actix_web::web;
use libsql_client::Value;
#[derive(Deserialize)]
pub struct GetApiPostParams {
    pub uid: String,
}

#[get("/api/post")]
async fn api_get_post(req: HttpRequest, params: web::Query<GetApiPostParams>) -> impl Responder {
    let uid = get_uid_from_jwt(req.cookie("chainpost_jwt").unwrap().value()).unwrap();
    let client = init_db().await;
    let result = get_posts_by_uid(&client, &params.uid).await;

    if let Ok(result_set) = result {
        let mut res_html = String::from("");
        for row in result_set.rows {
            let mut post_uid: Option<String> = None;
            let mut post_id: Option<usize> = None;
            let mut post_timestamp: Option<usize> = None;

            if let Value::Integer { value } = row.values[0].clone() {
                post_id = Some(value as usize);
            }
            if let Value::Text { value } = row.values[2].clone() {
                post_uid = Some(value);
            }
            if let Value::Integer { value } = row.values[3].clone() {
                post_timestamp = Some(value as usize);
            }

            match row.values[1].clone() {
                Value::Text { value } => {
                    let html = format!("<div id=\"post-{}\"class=\"max-w-sm mx-auto mb-5 bg-dark-200 rounded-lg overflow-hidden shadow-lg\"><div class=\"p-4 flex items-center justify-center h-40\"><p class=\"text-primary-500 text-center\">{}</p></div>", post_id.clone().map_or("error".to_string(), |v| v.to_string()), value);
                    for c in html.chars() {
                        res_html.push(c);
                    }
                    let additional_html = format!("<div class=\"px-4 py-2 bg-gray-100\"><div class=\"flex justify-between items-center\"><div><p class=\"text-xs whitespace-normal text-primary-500\">Posted by: <span class=\"font-semibold\">{}</span></p><p class=\"text-xs text-primary-500\">Posted at: <span class=\"font-semibold\">{}</span></p></div></div></div>", post_uid.clone().map_or("Not Found".to_string(), |v| v.to_string()), post_timestamp.map_or("Not Found".to_string(), |v| v.to_string()));
                    for c in additional_html.chars() {
                        res_html.push(c);
                    }

                    if post_uid == Some(uid.clone()) {
                        let delete_html = format!("<button hx-delete=\"/api/post?id={}\" hx-method=\"delete\" hx-target=\"#post-{}\" hx-swap=\"outerHTML\" class=\"bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded text-xs\">Delete</button>", post_id.map_or("None".to_string(), |v| v.to_string()), post_id.map_or("None".to_string(), |v| v.to_string()));
                        for c in delete_html.chars() {
                            res_html.push(c);
                        }
                    }

                    for c in "</div>".chars() {
                        res_html.push(c);
                    }
                }
                _ => return HttpResponse::InternalServerError().finish(),
            }
        }
        return HttpResponse::Ok().body(res_html);
    } else {
        return HttpResponse::InternalServerError().finish();
    }
}

#[derive(Deserialize)]
pub struct DeletePostParams {
    id: usize,
}
#[delete("/api/post")]
async fn api_delete_post(req: HttpRequest, params: web::Query<DeletePostParams>) -> impl Responder {
    if check_auth(&req) == false {
        return HttpResponse::Unauthorized().finish();
    }
    let client = init_db().await;
    let _ = delete_post(&client, params.id).await;
    return HttpResponse::Ok().finish();
}

#[derive(Deserialize)]
pub struct PostCsvParams {
    pub uid: String,
}

use actix_web::{
    http::header::ContentDisposition, http::header::DispositionParam, http::header::DispositionType,
};
use csv::Writer;

#[get("/api/postcsv")]
async fn api_post_csv(params: web::Query<PostCsvParams>) -> impl Responder {
    let client = init_db().await;
    let result_set = get_posts_by_uid(&client, &params.uid).await.unwrap();
    let mut wtr = Writer::from_writer(vec![]);
    wtr.write_record(&["ID", "Content", "Owner", "Creation Timestamp"])
        .unwrap();

    debug!("Attempting to create a CSV!");
    for row in result_set.rows {
        let mut post_uid: Option<String> = None;
        let mut post_id: Option<usize> = None;
        let mut post_timestamp: Option<usize> = None;
        let mut content: Option<String> = None;

        if let Value::Integer { value } = row.values[0].clone() {
            post_id = Some(value as usize);
        }
        if let Value::Text { value } = row.values[1].clone() {
            content = Some(value);
        }
        if let Value::Text { value } = row.values[2].clone() {
            post_uid = Some(value);
        }
        if let Value::Integer { value } = row.values[3].clone() {
            post_timestamp = Some(value as usize);
        }

        wtr.write_record(&[
            post_id.map_or("None".to_string(), |v| v.to_string()),
            content.map_or("None".to_string(), |v| v.to_string()),
            post_uid.map_or("None".to_string(), |v| v.to_string()),
            post_timestamp.map_or("None".to_string(), |v| v.to_string()),
        ])
        .unwrap();
    }

    wtr.flush().unwrap();
    let csv_data = wtr.into_inner().unwrap();
    debug!("{:?}", csv_data);
    return HttpResponse::Ok()
        .insert_header(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![
                DispositionParam::Name(String::from("file")),
                DispositionParam::Filename(String::from("posts.csv")),
            ],
        })
        .content_type("text/csv")
        .body(csv_data);
}
