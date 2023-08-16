#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/v1/user/info")]
pub async fn v1_user_info() -> impl Responder {
    HttpResponse::Ok().body("info")
}

#[post("/v1/user/settings")]
pub async fn v1_user_settings() -> impl Responder {
    HttpResponse::Ok().body("settings")
}