use actix_web::{App,HttpServer,middleware,web};
use handler::*;
use common::log::init_log;
use log::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_log();
    
    info!("start server!!!");
    HttpServer::new(
        ||{
            // handlers
            App::new().wrap(middleware::Logger::default())
            .service(student::index)
            .service(web::resource("/article").route(web::post().to(article::save_article)))
        }
    ).bind("127.0.0.1:8080")?.run().await
}