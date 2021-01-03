use crate::server::db::connect;
use sqlx::query;

pub async fn update_token(token: String, id: i32) -> Result<(), sqlx::Error> {
    let mut conn = connect().await;
    match query("UPDATE user SET token = $1 where id = $2")
        .bind(token)
        .bind(id)
        .execute(&mut conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn connect_user_by_token() {}
