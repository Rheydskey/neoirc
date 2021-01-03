use crate::server::db::connect;

pub async fn insert_user(name: String, password: String) -> Result<(), sqlx::Error> {
    let mut conn = connect().await;
    match sqlx::query("INSERT INTO user ( name, password ) VALUES ( $1, $2 )")
        .bind(name)
        .bind(password)
        .execute(&mut conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
