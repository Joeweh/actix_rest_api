use actix_web::{Responder, HttpResponse, post, put};

use crate::user::{User};

#[post("/api/users/register")]
pub(crate) async fn register(/*user_service: web::Data<UserService>*/) -> impl Responder {
    HttpResponse::NoContent()
}

#[post("/api/users/login")]
pub(crate) async fn login(/*user_service: web::Data<UserService>*/) -> impl Responder {
    let user: User = User {
        id: "testID".parse::<String>().unwrap(),
        email: "test@gmail.com".parse::<String>().unwrap(),
        password: "testPw".parse::<String>().unwrap(),
    };

    HttpResponse::Ok().json(user)
}

#[put("/api/users/{id}/change-password")]
pub(crate) async fn change_password(/*user_service: web::Data<UserService>*/) -> impl Responder {

    HttpResponse::NoContent()
}