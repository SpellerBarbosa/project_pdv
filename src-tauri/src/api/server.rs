use actix_web::{web, App, HttpServer};
use crate::api::routes::index::server_status;

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| { 
        App::new()
            .service(
                web::scope("/api")
                )
            .service(server_status)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
