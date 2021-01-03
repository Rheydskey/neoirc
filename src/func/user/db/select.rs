use crate::server::db::connect;
use sqlx::{query, Row};
use crate::func::user::user::User;

pub async fn select_user_by_id(id: i32) -> std::io::Result<()> {
    let mut conn = connect().await;

    query("SELECT * FROM user WHERE id = $1").bind(id).execute(&mut conn).await.expect("Error");
    Ok(())
}

pub async fn select_user_by_name(name: String) -> User {
    let mut conn = connect().await;
    let mut user = User::new();
    match query("SELECT * FROM user WHERE name = $1").bind(name).fetch_one(&mut conn).await {
        Ok(e) => {
            user.set_name(e.get::<&str, &str>("name").to_string()).set_nick(e.get::<&str, &str>("nick").to_string());
        },
        Err(e) => {
            println!("{:?}", e);
        }
    };
    println!("{:?}", user);
    user
}

pub async fn select_user_by_name_and_password(name: String, password: String) -> Result<User, sqlx::Error> {
    let mut conn = connect().await;
    match query("SELECT * FROM user WHERE name = $1 AND password = $2").bind(name).bind(password).fetch_one(&mut conn).await {
        Ok(e) => {
            let mut user = User::new();
            user.set_name(e.get::<&str, &str>("name").to_string()).set_id(e.get::<i32, &str>("id"));
            Ok(user)
        },
        Err(e) => {
            Err(e)
        }
    }
}