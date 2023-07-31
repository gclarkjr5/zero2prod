use zero2prod::router::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    run().await.expect("error starting server")
}