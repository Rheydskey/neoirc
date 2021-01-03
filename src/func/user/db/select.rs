use crate::func::user::user::User;
use crate::server::db::connect;
use sqlx::{query, Row};

pub async fn select_user_by_id(id: i32) -> std::io::Result<()> {
    let mut conn = connect().await;

    query("SELECT * FROM user WHERE id = $1")
        .bind(id)
        .execute(&mut conn)
        .await
        .expect("Error");
    Ok(())
}

pub async fn select_user_by_name(name: String) -> User {
    let mut conn = connect().await;
    let mut user = User::new();
    match query("SELECT * FROM user WHERE name = $1")
        .bind(name)
        .fetch_one(&mut conn)
        .await
    {
        Ok(e) => {
            user.set_name(e.get::<&str, &str>("name").to_string())
                .set_nick(e.get::<&str, &str>("nick").to_string());
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };
    println!("{:?}", user);
    user
}

pub async fn select_user_by_name_and_password(
    name: String,
    password: String,
) -> Result<User, sqlx::Error> {
    let mut conn = connect().await;
    match query("SELECT * FROM user WHERE name = $1 AND password = $2")
        .bind(name)
        .bind(password)
        .fetch_one(&mut conn)
        .await
    {
        Ok(e) => {
            let mut user = User::new();
            user.set_name(e.get::<&str, &str>("name").to_string())
                .set_id(e.get::<i32, &str>("id"));
            Ok(user)
        }
        Err(e) => Err(e),
    }
}

pub async fn select_user_connect() -> Vec<User> {
    let mut vec: Vec<User> = Vec::new();
    let mut conn = connect().await;
    let fetched = query("SELECT * FROM user WHERE connected = true ")
        .fetch_all(&mut conn)
        .await
        .expect("Error");
    for fetch in fetched {
        let mut user = User::new();
        user.set_name(fetch.get::<&str, &str>("name").to_string());
        user.set_nick(fetch.get::<&str, &str>("nick").to_string());
        user.set_id(fetch.get::<i32, &str>("id"));
        vec.push(user)
    }
    vec
}

pub async fn valid_token(token: String) -> bool {
    let mut conn = connect().await;
    if let Ok(e) = query("SELECT * FROM user WHERE token = $1").bind(token).fetch_one(&mut conn).await {
        if !e.is_empty() {
            true
        } else {
            false
        }
    } else {
        false
    }
}