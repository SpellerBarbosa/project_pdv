use actix_web::{web, App, HttpServer};
use crate::api::routes::index::server_status;
use crate::api::routes::signup::register_user;
use crate::api::routes::has_admin::has_admin;

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| { 
        App::new()
            .service(
                web::scope("/api")
                            .service(register_user)
                            .service(has_admin)
                )
            .service(server_status)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
