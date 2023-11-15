use crate::db::init_db;
use crate::db::users;
use actix_web::{cookie::Cookie, get, web, HttpRequest, HttpResponse, Responder};
use jsonwebtoken::EncodingKey;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCardanoValidateSignatureResponse {
    pub uid: String,
    pub valid_signature: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiAuthCardanoParams {
    pub signature: String,
    pub key: String,
}

#[get("/api/auth/cardano")]
pub async fn api_auth_cardano(
    req: HttpRequest,
    params: web::Query<ApiAuthCardanoParams>,
) -> impl Responder {
    info!("/api/auth/cardano REQUEST {:?}", &req);
    let client = reqwest::Client::new();
    debug!("sending with body: {:?}", &params);
    let res = client
        .post("https://us-central1-chainpost.cloudfunctions.net/cardano-validate-signature")
        .json(&params.into_inner())
        .send()
        .await;
    if let Ok(res) = res {
        let res = res.json::<FunctionCardanoValidateSignatureResponse>().await;
        debug!("{:?}", &res);
        if let Ok(res) = res {
            if res.valid_signature {
                if let Ok(jwt) = mint_jwt(res.uid.as_str()) {
                    let client = init_db().await;
                    let db_result = users::get_user_by_uid(&client, &res.uid).await;
                    if let Ok(result) = db_result {
                        // Create new user
                        if result.rows.len() == 0 {
                            let result = users::create_user(
                                &client,
                                res.uid,
                                None,
                                None,
                                chrono::Utc::now().timestamp() as usize,
                            )
                            .await;
                            if let Ok(result) = result {
                                debug!("New user created! {:?}", result);
                            } else {
                                error!("Error creating user!");
                                return HttpResponse::InternalServerError().finish();
                            }
                        }
                    } else {
                        error!("Error querying db for user by uid.");
                        return HttpResponse::InternalServerError().finish();
                    }
                    let jwt_cookie = Cookie::build("chainpost_jwt", jwt.as_str())
                        .http_only(true)
                        .same_site(actix_web::cookie::SameSite::Strict)
                        .path("/")
                        .finish();
                    return HttpResponse::Ok().cookie(jwt_cookie).finish();
                } else {
                    error!("/api/auth/cardano error minting the jwt.")
                }
            } else {
                debug!("/api/auth/cardano invalid signature.");
                return HttpResponse::BadRequest().finish();
            }
        } else {
            error!("/api/auth/cardano error parsing response from cardano-validate-signature.")
        }
    } else {
        error!("/api/auth/cardano error returned from cardano-validate-signature.");
    }
    return HttpResponse::InternalServerError().finish();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
    pub nbf: usize,
    pub aud: String,
}

fn mint_jwt(uid: &str) -> Result<String, Box<dyn std::error::Error>> {
    debug!("Minting JWT for {}", uid);
    let now = chrono::Utc::now();
    let claims = TokenClaims {
        sub: String::from(uid),
        iss: String::from("chainpost"),
        exp: (now + chrono::Duration::hours(12)).timestamp() as usize,
        iat: now.timestamp() as usize,
        nbf: now.timestamp() as usize,
        aud: String::from("https://chainpost.social/"),
    };
    let jwt_secret_key = std::env::var("JWT_SECRET_KEY")?;
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret_key.as_ref()),
    )?;
    return Ok(token);
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{http, test, App};
    use jsonwebtoken::{decode, DecodingKey, Validation};

    #[actix_web::test]
    async fn test_api_auth_cardano() {
        let app = test::init_service(App::new().service(api_auth_cardano)).await;
        let req = test::TestRequest::get()
            .uri("/api/auth/cardano")
            .param("signature", "INVALID_SIGNATURE")
            .param("key", "INVALID_KEY")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            http::StatusCode::BAD_REQUEST,
            "Invalid signature in keys returns a 400"
        );
    }

    #[test]
    async fn test_mint_jwt() {
        let result = mint_jwt("Super Fake ID");
        match result {
            Ok(jwt) => {
                let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
                validation.insecure_disable_signature_validation();
                let decoded_token = decode::<TokenClaims>(
                    &jwt,
                    &DecodingKey::from_secret(std::env::var("JWT_SECRET_KEY").unwrap().as_bytes()),
                    &validation,
                )
                .unwrap();
                assert_eq!(&decoded_token.claims.iss, "chainpost");
                assert_eq!(
                    decoded_token.claims.exp,
                    decoded_token.claims.iat + chrono::Duration::hours(12).num_seconds() as usize
                );
                assert_eq!(decoded_token.claims.nbf, decoded_token.claims.iat);
            }
            Err(err) => {
                assert!(false, "mint_jwt() returned an error: {}", err);
            }
        }
    }
}
