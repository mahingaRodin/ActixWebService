use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(health_check))
    );
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "healthy", "service": "task_service"}))
}