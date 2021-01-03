use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use super::db::{insert::insert_user, model::UserPassword, select::select_user_by_name};
use crate::func::hash_password;
use crate::func::user::db::delete::delete_user_by_token;
use crate::func::user::db::select::{select_user_by_name_and_password, select_user_connect, valid_token};
use crate::func::user::db::update::{update_token, update_connect_status_by_token, remove_token};
use rand::Rng;
use actix_web::error::PayloadError::Http2Payload;

#[derive(Clone, Deserialize, Debug)]
pub struct User {
    id: i32,
    name: String,
    nick: String,
}

impl User {
    pub fn new() -> Self {
        User {
            id: 0,
            name: String::new(),
            nick: String::new(),
        }
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_nick(&self) -> String {
        self.nick.clone()
    }
    pub fn set_id(&mut self, id: i32) -> &mut User {
        self.id = id;
        self
    }
    pub fn set_name(&mut self, name: String) -> &mut User {
        self.name = name;
        self
    }
    pub fn set_nick(&mut self, nick: String) -> &mut User {
        self.nick = nick;
        self
    }
    pub fn generate_token() -> String {
        let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456789";
        let mut rng = rand::thread_rng();
        (0..30)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect()
    }
}

pub async fn create_user(req: HttpRequest, json: web::Json<UserPassword>) -> HttpResponse {
    let user = select_user_by_name(json.0.get_name()).await;
    if !user.get_name().is_empty() {
        return HttpResponse::Ok().body("User already Exist");
    }
    let hash = hash_password(json.0.get_password().clone());
    insert_user(json.0.get_name(), hash).await;
    HttpResponse::Ok().body("Ok working")
}

pub async fn delete_user(req: HttpRequest, json: web::Json<UserPassword>) -> HttpResponse {
    if let Some(e) = req.headers().get("Token") {
        delete_user_by_token(e.to_str().unwrap_or(&"").to_string()).await;
        HttpResponse::Ok().body("Ok working")
    } else {
        HttpResponse::Ok().body("Hey Where is the token")
    }
}

pub async fn select_user(req: HttpRequest, json: web::Json<UserPassword>) -> HttpResponse {
    select_user_by_name(json.get_name()).await;
    HttpResponse::Ok().body("Ok working")
}
pub async fn login_user(req: HttpRequest, json: web::Json<UserPassword>) -> HttpResponse {
    let hash = hash_password(json.0.get_password().clone());
    let user = select_user_by_name_and_password(json.0.get_name(), hash)
        .await
        .expect("Error");
    let token = User::generate_token();
    if let Ok(_) = update_token(token.clone(), user.get_id()).await {
        update_connect_status_by_token(true, token.clone()).await;
        HttpResponse::Ok().body(token.clone())
    } else {
        HttpResponse::Ok().body(r#"{"result":"false", "message":"error on token update"}"#)
    }
}


pub async fn logout_user(req: HttpRequest) -> HttpResponse {
    if let Some(e) = req.headers().get("token") {
        let token = e.to_str().expect("Error").to_string();
        if valid_token(token.clone()).await {
            update_connect_status_by_token(false, token.clone()).await;
            remove_token(token.clone()).await;
            HttpResponse::Ok().body("Logout succesfully")
        } else {
            HttpResponse::Ok().body("bad token")
        }
    } else {
        HttpResponse::Ok().body("No token")
    }
}
pub async fn list_user() -> HttpResponse {
    let user = select_user_connect().await;
    HttpResponse::Ok().body(format!("{:?}", user))
}

