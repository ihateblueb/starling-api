#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[post("/v1/receive/status")]
pub async fn v1_receive_status() -> impl Responder {
    println!("received status");
    HttpResponse::Ok().body("0")
}

#[post("/v1/receive/config")]
pub async fn v1_receive_config() -> impl Responder {
    println!("received config");
    HttpResponse::Ok().body("config")
}