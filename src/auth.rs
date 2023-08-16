#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/v1/auth/validation")]
pub async fn v1_auth_validation() -> impl Responder {
    HttpResponse::Ok().body("validation")
}

#[post("/v1/auth/login")]
pub async fn v1_auth_login() -> impl Responder {
    HttpResponse::Ok().body("login")
}

#[post("/v1/auth/logout")]
pub async fn v1_auth_logout() -> impl Responder {
    HttpResponse::Ok().body("logout")
}