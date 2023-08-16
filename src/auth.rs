#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/v1/auth/validation")]
pub async fn v1_auth_validation() -> impl Responder {
    HttpResponse::Ok().body("validation")
}

#[get("/v1/auth/session")]
pub async fn v1_auth_session() -> impl Responder {
    HttpResponse::Ok().body("session")
}

#[post("/v1/auth/login")]
pub async fn v1_auth_login() -> impl Responder {
    HttpResponse::Ok().body("login")
}

#[post("/v1/auth/logout")]
pub async fn v1_auth_logout() -> impl Responder {
    HttpResponse::Ok().body("logout")
}

#[post("/v1/auth/register")]
pub async fn v1_auth_register() -> impl Responder {
    HttpResponse::Ok().body("register")
}