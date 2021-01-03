use crate::server::db::connect;

pub async fn delete_user(id: i32) -> std::io::Result<()> {
    let mut conn = connect().await;
    sqlx::query("DELETE user WHERE id = $1")
        .bind(id)
        .execute(&mut conn)
        .await
        .expect("Error");
    Ok(())
}

pub async fn delete_user_by_token(token: String) -> std::io::Result<()> {
    let mut conn = connect().await;
    sqlx::query("DELETE FROM user WHERE token = $1")
        .bind(token)
        .execute(&mut conn)
        .await
        .expect("Error");
    Ok(())
}
