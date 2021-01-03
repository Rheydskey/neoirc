use sqlx::Connection;

pub mod create;

pub async fn connect() -> sqlx::SqliteConnection {
    sqlx::SqliteConnection::connect("./db.sql")
        .await
        .expect("Error")
}
