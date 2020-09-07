use actix_web::{App,HttpServer,middleware};
// use model::mongo;
use handler::student;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // mongo::init();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(||App::new().wrap(middleware::Logger::default()).service(student::index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}