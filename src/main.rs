mod api;
mod repository;
mod model;

use actix_web::{HttpServer, App, middleware::Logger, web};
use log::info;
use crate::repository::create_dynamodb_client;
use crate::repository::task_repository::TaskRepository;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    info!("Starting Actix web server...");

    //creating dynamo client
    let dynamodb_client = create_dynamodb_client().await;
    let task_repository = TaskRepository::new(dynamodb_client);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(task_repository.clone()))
            .configure(api::configure_routes)
    })
    .bind(("127.0.0.1",8081))?
    .run()
    .await
}