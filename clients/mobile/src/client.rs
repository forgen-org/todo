use crate::dto::ErrorDTO;
use crate::{dto::TodoListDTO, runtime::Runtime};
use application::command::Command;
use application::query::GetTodoListQuery;
use framework::*;
use std::sync::Arc;

#[derive(uniffi::Object)]
pub struct Client {
    runtime: Runtime,
}

#[uniffi::export]
impl Client {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            runtime: Runtime::default(),
        })
    }

    pub async fn add_task(&self, name: String) -> Result<(), ErrorDTO> {
        Command::AddTask { name }
            .execute(&self.runtime)
            .await
            .map_err(ErrorDTO::from)
    }

    pub async fn remove_task(&self, index: u32) -> Result<(), ErrorDTO> {
        Command::RemoveTask {
            index: index as usize,
        }
        .execute(&self.runtime)
        .await
        .map_err(ErrorDTO::from)
    }

    pub async fn complete_task(&self, index: u32) -> Result<(), ErrorDTO> {
        Command::CompleteTask {
            index: index as usize,
        }
        .execute(&self.runtime)
        .await
        .map_err(ErrorDTO::from)
    }

    pub async fn get_todo_list(&self) -> Result<TodoListDTO, ErrorDTO> {
        GetTodoListQuery {}
            .execute(&self.runtime)
            .await
            .map_err(ErrorDTO::from)
            .map(TodoListDTO::from)
    }
}
