use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::model::{Task, CreateTaskRequest, UpdateTaskRequest};
use crate::repository::task_repository::TaskRepository;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .route("", web::get().to(get_tasks))
            .route("", web::post().to(create_task))
            .route("/{id}", web::get().to(get_task))
            .route("/{id}", web::put().to(update_task))
            .route("/{id}", web::delete().to(delete_task))
    );
}

pub async fn get_tasks(task_repo: web::Data<TaskRepository>) -> impl Responder {
    match task_repo.list_tasks().await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            log::error!("Failed to fetch tasks: {}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch tasks"}))
        }
    }
}

pub async fn get_task (
    task_repo: web::Data<TaskRepository>,
    path: web::Path<(String,)>,
) -> impl Responder {
    let (task_id,) = path.into_inner();
    match task_repo.get_task(task_id.to_string()).await {
        Ok(task) => HttpResponse::Ok().json(task),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "Task not found"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": format!("Failed to fetch task: {}", e)})),
    }
}


pub async fn create_task(
    task_repo: web::Data<TaskRepository>,
    create_request: web::Json<CreateTaskRequest>,
) -> impl Responder {
    let task = Task::new(create_request.into_inner());
    match task_repo.create_task(&task).await {
        Ok(()) => HttpResponse::Created().json(task),
        Err(e) => {
            log::error!("Failed to create task: {}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to create task"}))
        }
    }
}

pub async fn update_task(
    task_repo: web::Data<TaskRepository>,
    path: web::Path<String>,
    update_request: web::Json<UpdateTaskRequest>,
) -> impl Responder {
    let task_id = path.into_inner();

    match task_repo.get_task(task_id).await {
        Ok(Some(mut task)) => {
            task.update(update_request.into_inner());

            match task_repo.update_task(&task).await {
                Ok(()) => HttpResponse::Ok().json(task),
                Err(e) => {
                    log::error!("Failed to update task: {}", e);
                    HttpResponse::InternalServerError().json(json!({"error": "Failed to update task"}))
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "Task not found"})),
        Err(e) => {
            log::error!("Failed to fetch task for update: {}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to update task"}))
        }
    }
}


pub async fn delete_task(
task_repo: web::Data<TaskRepository>,
path: web::Path<String>,
) -> impl Responder {
    let task_id = path.into_inner();

    match task_repo.get_task(task_id).await {
    Ok(..) => HttpResponse::NoContent().finish(),
        Err(e) => {
            log::error!("Failed to delete task: {}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Failed to delete task"}))
        }
    }
}
