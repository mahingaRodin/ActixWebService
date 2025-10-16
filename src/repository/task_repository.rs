use aws_sdk_dynamodb::{Client, types::AttributeValue};
use crate::model::Task;
use crate::repository::get_table_name;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TaskRepository {
    client: Client,
    table_name: String,
}

impl TaskRepository {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            table_name: get_table_name(),
        }
    }

    pub async fn create_task(&self, task: &Task) -> Result<(), Box<dyn std::error::Error>> {
        let request = self.client
            .put_item()
            .table_name(&self.table_name)
            .item("id", AttributeValue::S(String::from(task.id.clone())))
            .item("title", AttributeValue::S(task.title.clone()))
            .item("description", AttributeValue::S(task.description.clone()))
            .item("completed", AttributeValue::Bool(task.completed))
            .item("created_at", AttributeValue::S(task.created_at.to_rfc3339()))
            .item("updated_at", AttributeValue::S(task.updated_at.to_rfc3339()))
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_task(&self, task_id: (String)) -> Result<Option<Task>, Box<dyn std::error::Error>> {
        let request = self.client
            .get_item()
            .table_name(&self.table_name)
            .key("id", AttributeValue::S(task_id.to_string()));

        let response = request.send().await?;

        if let Some(item) = response.item {
            let task = Self::item_to_task(&item)?;
            Ok(Some(task))
        } else {
            Ok(None)
        }
    }

    pub async fn list_tasks(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let request = self.client
            .scan()
            .table_name(&self.table_name);
        let response = request.send().await?;
        let mut tasks = Vec::new();
        if let Some(items) = response.items {
            for item in items {
                if let Ok(task) = Self::item_to_task(&item) {
                    tasks.push(task);
                }
            }
        }
        Ok(tasks)
    }

    pub async fn update_task(&self, task: &Task) -> Result<(), Box<dyn std::error::Error>> {
        let request = self.client
            .put_item()
            .table_name(&self.table_name)
            .item("id", AttributeValue::S(String::from(task.id.clone())))
            .item("title", AttributeValue::S(task.title.clone()))
            .item("description", AttributeValue::S(task.description.clone()))
            .item("completed", AttributeValue::Bool(task.completed))
            .item("created_at", AttributeValue::S(task.created_at.to_rfc3339()))
            .item("updated_at", AttributeValue::S(task.updated_at.to_rfc3339()));

        request.send().await?;
        Ok(())
    }

    pub async fn delete_task(&self, task_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let request = self.client
            .delete_item()
            .table_name(&self.table_name)
            .key("id", AttributeValue::S(task_id.to_string()));

        request.send().await?;
        Ok(())
    }


    fn item_to_task(item: &HashMap<String, AttributeValue>) -> Result<Task, Box<dyn std::error::Error>> {
        use chrono::TimeZone;

        let id = item.get("id")
            .and_then(|v| v.as_s().ok())
            .ok_or("Missing id")?
            .to_string();

        let title = item.get("title")
            .and_then(|v| v.as_s().ok())
            .ok_or("Missing title")?
            .to_string();

        let description = item.get("description")
            .and_then(|v| v.as_s().ok())
            .map(|s| if s.is_empty() { None } else { Some(s.to_string()) })
            .unwrap_or(None);

        let completed = item.get("completed")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(&false);

        let created_at_str = item.get("created_at")
            .and_then(|v| v.as_s().ok())
            .ok_or("Missing created_at")?;
        let created_at = DateTime::parse_from_rfc3339(created_at_str)?
            .with_timezone(&Utc);

        let updated_at_str = item.get("updated_at")
            .and_then(|v| v.as_s().ok())
            .ok_or("Missing updated_at")?;
        let updated_at = DateTime::parse_from_rfc3339(updated_at_str)?
            .with_timezone(&Utc);

        Ok(Task {
            id: Uuid::parse_str(&id)?,
            title,
            description: description.expect("REASON"),
            completed: *completed,
            created_at,
            updated_at,
        })
    }
}