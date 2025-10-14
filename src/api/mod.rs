pub mod task;
pub mod health;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // .configure(task::configure_routes)
            .configure(health::configure_routes)
    );
}