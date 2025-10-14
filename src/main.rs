mod api;
mod repository;
mod model;

// use api::task::get_task;
use actix_web::{HttpServer, App, middleware::Logger};
use log::info;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    info!("Starting Actix web server...");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            // .service(
            //     // get_task
            // )
    })
    .bind(("127.0.0.1",8081))?
    .run()
    .await
}