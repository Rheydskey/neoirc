use crate::server::db::connect;

pub async fn create_user_db() {
    let mut conn = connect().await;
    sqlx::query(        "CREATE TABLE IF NOT EXISTS user (
        id              INTEGER PRIMARY KEY,
        name            TEXT NOT NULL,
        nick            TEXT,
        password        TEXT NOT NULL,
        token           TEXT
        )").execute(&mut conn).await.expect("Error");
}