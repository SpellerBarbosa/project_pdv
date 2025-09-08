use actix_web::{http, web, App, HttpServer};
use crate::api::routes::index::server_status;
use crate::api::routes::signup::register_user;
use crate::api::routes::has_admin::has_admin;
use actix_cors::Cors;

pub async fn start_server() -> std::io::Result<()> {
    //criar o servidor
    HttpServer::new(|| { 
        //cria a configuração do cors
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b".localhost:3000")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        //configura as rotas e a porta que o servidor vai utilizar
        App::new()
            .wrap(cors)
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
