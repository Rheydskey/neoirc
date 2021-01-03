use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde::Deserialize;

use super::db::{insert::insert_user, model::UserPassword, select::select_user_by_name};
use crate::func::user::db::select::select_user_by_name_and_password;
use crate::func::hash_password;
use rand::Rng;
use crate::func::user::db::update::update_token;

#[derive(Clone,Deserialize, Debug)]
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
        return HttpResponse::Ok().body("User already Exist")
    }
    let hash = hash_password(json.0.get_password().clone());
    insert_user(json.0.get_name(), hash).await;
    HttpResponse::Ok().body("Ok working")
}

pub async fn delete_user(req: HttpRequest, json: web::Json<UserPassword>) -> HttpResponse {
    insert_user(json.0.get_name(),json.0.get_password()).await;
    HttpResponse::Ok().body("Ok working")
}

pub async fn select_user(req: HttpRequest, json: web::Json<UserPassword>) -> HttpResponse {
    select_user_by_name(json.get_name()).await;
    HttpResponse::Ok().body("Ok working")
}
pub async fn login_user(req: HttpRequest, json: web::Json<UserPassword>) -> HttpResponse {
    let hash = hash_password(json.0.get_password().clone());
    let user = select_user_by_name_and_password(json.0.get_name(), hash).await.expect("Error");
    let token = User::generate_token();
    if let Ok(_)  = update_token(token.clone(), user.get_id()).await {
        HttpResponse::Ok().body(token.clone())
    } else {
        HttpResponse::Ok().body(r#"{"result":"false", "message":"error on token update"}"#)
    }

}