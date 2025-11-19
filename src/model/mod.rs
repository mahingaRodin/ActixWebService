// use std::option::Option;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(create_request: CreateTaskRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: create_request.title,
            description: create_request.description,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }


    pub fn update(&mut self, update_task_request: UpdateTaskRequest) {
        let now = Utc::now();
      //used direct assignment
        self.title = update_task_request.title;
        self.description = update_task_request.description;
        self.completed = update_task_request.completed;
        self.updated_at = now;
    }
}