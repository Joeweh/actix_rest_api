mod user;
mod user_service;
mod user_routes;

mod utils;
mod middleware;

use actix_web::{web, App, HttpServer, get, Responder, HttpResponse};

use user_routes::{register, login, change_password};
use user_service::UserService;

use std::env;
use actix_cors::Cors;
use actix_web::middleware::Logger;

use sqlx::mysql::MySqlPoolOptions;
use env_logger::Env;

#[get("/api")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("SERVER ONLINE")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(env::var("DB_URL").unwrap().as_str())
        .await.unwrap();


    let port: u16 = match env::var("PORT") {
        Ok(port_string) => port_string.parse::<u16>().unwrap(),
        Err(_error) => 8080
    };

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        Cors::permissive();
        App::new()
            .app_data(web::Data::new(UserService::new(pool.clone())))

            .service(register)
            .service(login)
            .service(change_password)

            .service(health_check)

            .wrap(Logger::new("  Remote IP-Address: %a  Request Info: \"%r\"  Response: %s"))
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}