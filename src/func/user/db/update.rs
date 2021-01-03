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

pub async fn update_connect_status_by_token(status: bool,token: String) -> Result<(), sqlx::Error>{
    let mut conn = connect().await;
    match query("UPDATE user SET connected = $1 WHERE token = $2").bind(status).bind(token).execute(&mut conn).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

pub async fn remove_token(token: String) -> Result<(), sqlx::Error>{
    let mut conn = connect().await;
    match query("UPDATE user SET token = $1 WHERE token = $2").bind("").bind(token).execute(&mut conn).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}
