// pub mod memory_repository;
// pub mod dynamodb_repository;
//
// pub use memory_repository::MemoryTaskRepository;
// pub use dynamodb_repository::DynamoDbTaskRepository;

pub(crate) mod task_repository;
mod dynamodb_repository;

use aws_sdk_dynamodb::Client;

pub async fn create_dynamodb_client() -> Client {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    Client::new(&config)
}

pub fn get_table_name() -> String {
    std::env::var("DYNAMODB_TABLE").unwrap_or_else(|_| "tasks".to_string())
}