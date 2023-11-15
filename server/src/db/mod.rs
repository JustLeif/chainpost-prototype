use libsql_client::Config;
use log::info;
use std::env;
pub mod posts;
pub mod users;

pub async fn init_db() -> libsql_client::Client {
    let libsql_url = env::var("LIBSQL_CLIENT_URL").expect("Error finding database URL env var.");
    let auth_token =
        env::var("LIBSQL_CLIENT_TOKEN").expect("Error finding database auth token env var.");
    info!("Initializing db connection to: {}", libsql_url.as_str());
    return libsql_client::Client::from_config(Config {
        url: url::Url::parse(libsql_url.as_str()).unwrap(),
        auth_token: Some(String::from(auth_token.as_str())),
    })
    .await
    .unwrap();
}
