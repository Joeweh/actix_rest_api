use actix_web::{Responder, HttpResponse, post, put, web};
use serde_json::Value;
use sqlx::Row;
use crate::middleware::auth::AuthToken;

use crate::user::UserDAO;
use crate::user_service::UserService;
use crate::utils::jwt::issue_access_token;

#[post("/api/users/register")]
pub(crate) async fn register(mut new_user: web::Json<UserDAO>, user_service: web::Data<UserService>) -> impl Responder {
    let result = user_service.register(&mut new_user).await;

    if result.is_err() {
        HttpResponse::Conflict();
    }

    HttpResponse::Created()
}

#[post("/api/users/login")]
pub(crate) async fn login(credentials: web::Json<UserDAO>, user_service: web::Data<UserService>) -> impl Responder {
    let result = user_service.login(&credentials.email, &credentials.password).await;

    if result.is_err() {
        HttpResponse::Unauthorized();
    }

    let token = issue_access_token(result.unwrap().get("id"));

    HttpResponse::Ok().body(token)
}

#[put("/api/users/change-password")]
pub(crate) async fn change_password(token: AuthToken, body: String, user_service: web::Data<UserService>) -> impl Responder {
    let new_password: Value = serde_json::from_str(body.as_str()).unwrap();

    let result = user_service.change_password(&token.id.to_string(), &new_password["new_password"].as_str().unwrap().to_string()).await;

    if result.is_err() {
        HttpResponse::Conflict();
    }

    HttpResponse::NoContent()
}