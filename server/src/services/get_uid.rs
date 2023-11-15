use crate::services::check_auth::decode_jwt;

pub fn get_uid_from_jwt(jwt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let claims = decode_jwt(jwt, std::env::var("JWT_SECRET_KEY")?.as_str())?;
    return Ok(claims.sub);
}
