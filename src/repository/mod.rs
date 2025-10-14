pub mod task_repository;

use aws_sdk_dynamodb::Client;
use std::env;

pub async fn create_dynamodb_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

pub fn get_table_name() -> String {
    env::var("DYNAMODB_TABLE").unwrap_or_else(|_| "tasks".to_string())
}