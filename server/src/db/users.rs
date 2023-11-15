use libsql_client::{args, Client, ResultSet, Statement};

pub struct User {
    pub uid: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub registration_timestamp: usize,
}

pub async fn create_user(
    client: &Client,
    uid: String,
    display_name: Option<String>,
    email: Option<String>,
    registration_timestamp: usize,
) -> Result<ResultSet, Box<dyn std::error::Error>> {
    let set = client.execute(
        Statement::with_args(
            "INSERT INTO users (uid, display_name, email, registration_timestamp) VALUES (?, ?, ?, ?)", 
            args!(uid, display_name, email, registration_timestamp)
        )
    ).await?;
    return Ok(set);
}

pub async fn get_user_by_uid(
    client: &Client,
    uid: &str,
) -> Result<ResultSet, Box<dyn std::error::Error>> {
    let set = client
        .execute(Statement::with_args(
            "SELECT * FROM users WHERE uid = ?",
            args!(uid),
        ))
        .await?;
    return Ok(set);
}
