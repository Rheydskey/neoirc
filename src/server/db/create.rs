use async_std::stream::StreamExt;
use sqlx::{Connection, Sqlite, SqliteConnection};
use std::path::PathBuf;

use crate::func::user::db::create::create_user_db;
pub async fn create_database() {
    let read = read_dir_to_vec(".".parse().unwrap()).await;
    if !read.contains(&"db.sql".to_string()) {
        async_std::fs::write("./db.sql", "").await.expect("Error");
    }
    SqliteConnection::connect("./db.sql").await.expect("Error");
    create_user_db().await;
}

pub async fn read_dir_to_vec(p: PathBuf) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut ee = async_std::fs::read_dir(p).await.expect("err");
    while let Some(some) = ee.next().await {
        match some {
            Ok(e) => {
                result.push(String::from(e.file_name().to_str().expect("err")));
            }
            Err(_) => {
                panic!()
            }
        }
    }
    result
}
