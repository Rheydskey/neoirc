mod server;

use crate::func::user::user::{list_user, login_user, logout_user};
use crate::server::db::create::create_database;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use func::user::user::{create_user, delete_user};

mod func;

async fn default(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    create_database().await;
    println!("Running......");
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/user")
                    .route("/create", web::post().to(create_user))
                    .route("/login", web::post().to(login_user))
                    .route("/delete", web::post().to(delete_user))
                    .route("/update", web::get().to(default))
                    .route("/list", web::get().to(list_user))
                    .route("/logout", web::get().to(logout_user))
                    .default_service(web::get().to(default)),
            )
            .service(
                web::scope("/message")
                    .route("/send", web::get().to(default))
                    .route("/delete/{id}", web::get().to(default))
                    .route("/update/{id}", web::get().to(default)),
            )
            .service(
                web::scope("/channel")
                    .route("/add/{id}", web::get().to(default))
                    .route("/update/{id}", web::get().to(default))
                    .route("/delete/{id}", web::get().to(default)),
            )
            .default_service(web::get().to(default))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
