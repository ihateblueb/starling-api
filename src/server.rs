#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/v1/server/info")]
pub async fn v1_server_info() -> impl Responder {
    HttpResponse::Ok().body("info")
}

#[post("/v1/server/settings")]
pub async fn v1_server_settings() -> impl Responder {
    HttpResponse::Ok().body("settings")
}

// get status of queue
#[get("/v1/server/queue/status")]
pub async fn v1_server_queue_status() -> impl Responder {
    HttpResponse::Ok().body("queue")
}

// get content after
#[get("/v1/server/queue/content")]
pub async fn v1_server_queue_content() -> impl Responder {
    HttpResponse::Ok().body("queue")
}

// remove 1 item from queue
#[post("/v1/server/queue/content/remove")]
pub async fn v1_server_queue_content_remove() -> impl Responder {
    HttpResponse::Ok().body("queue")
}