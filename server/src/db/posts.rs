use libsql_client::{args, Client, ResultSet, Statement};

pub struct Post {
    pub id: usize,
    pub content: String,
    pub uid: String,
    pub creation_timestamp: usize,
}

pub async fn create_post(
    client: &Client,
    content: String,
    uid: String,
) -> Result<ResultSet, Box<dyn std::error::Error>> {
    let creation_timestamp = chrono::Utc::now().timestamp() as usize;
    let set = client
        .execute(Statement::with_args(
            "INSERT INTO posts (content, uid, creation_timestamp) VALUES (?, ?, ?)",
            args!(content, uid, creation_timestamp),
        ))
        .await?;
    return Ok(set);
}

pub async fn get_posts_by_uid(
    client: &Client,
    uid: &str,
) -> Result<ResultSet, Box<dyn std::error::Error>> {
    let set = client
        .execute(Statement::with_args(
            "SELECT * FROM posts WHERE uid = ? ORDER BY creation_timestamp DESC",
            args!(uid),
        ))
        .await?;
    return Ok(set);
}

pub async fn delete_post(
    client: &Client,
    id: usize,
) -> Result<ResultSet, Box<dyn std::error::Error>> {
    let set = client
        .execute(Statement::with_args(
            "DELETE FROM posts WHERE id = ?",
            args!(id),
        ))
        .await?;
    return Ok(set);
}
