use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;

use crate::handlers::{greet, health_check};


pub async fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // .service(hello)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/greet/{name}", web::get().to(greet))
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(server)
}