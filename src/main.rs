#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// for interaction internally
mod auth;
mod user;
mod server;

// for interaction with servers
mod receive;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // auth
            .service(auth::v1_auth_validation)
            .service(auth::v1_auth_login)
            .service(auth::v1_auth_logout)

            // user
            .service(user::v1_user_info)
            .service(user::v1_user_settings)

            // server
            .service(server::v1_server_info)
            .service(server::v1_server_settings)
            .service(server::v1_server_queue_status)
            .service(server::v1_server_queue_content)
            .service(server::v1_server_queue_content_remove)

            // receive
            .service(receive::v1_receive_status)
            .service(receive::v1_receive_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}