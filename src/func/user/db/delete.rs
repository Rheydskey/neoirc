use crate::server::db::connect;

pub async fn delete_user(id: i32) -> std::io::Result<()> {
    let mut conn = connect().await;
    sqlx::query("DELETE user WHERE id = $1").bind(id).execute(&mut conn).await.expect("Error");
    Ok(())
}