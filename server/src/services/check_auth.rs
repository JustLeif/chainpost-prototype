use crate::api::auth::TokenClaims;
use actix_web::HttpRequest;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use log::debug;

pub fn check_auth(req: &HttpRequest) -> bool {
    if let Some(jwt) = req.cookie("chainpost_jwt") {
        if let Ok(valid) = valid_jwt(jwt.value()) {
            if valid {
                return true;
            } else {
                debug!("The JWT was invalid!");
                return false;
            }
        } else {
            debug!("An error occured when parsing the JWT!");
            return false;
        }
    } else {
        debug!("Could not find jwt chainpost_jwt! Redirecting to /app/auth");
        return false;
    }
}

fn valid_jwt(jwt: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if let Ok(claims) = decode_jwt(jwt, std::env::var("JWT_SECRET_KEY")?.as_str()) {
        if claims.exp >= chrono::Utc::now().timestamp() as usize {
            return Ok(true);
        } else {
            return Ok(false);
        }
    } else {
        return Ok(false);
    }
}

pub fn decode_jwt(
    token: &str,
    secret_key: &str,
) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256); // Assuming HS256 algorithm
    decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
}
