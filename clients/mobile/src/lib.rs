use application::command::Command;
use application::projection::TodoList;
use application::query::GetTodoListQuery;
use framework::*;
use std::sync::Arc;

mod runtime;

uniffi::setup_scaffolding!();

#[derive(uniffi::Object)]
pub struct Client {
    runtime: runtime::Runtime,
}

#[uniffi::export]
impl Client {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            runtime: runtime::Runtime::default(),
        })
    }

    pub async fn add_task(&self, name: String) {
        Command::AddTask { name }
            .execute(&self.runtime)
            .await
            .unwrap();
    }

    pub async fn get_todolist(&self) -> TodoList {
        GetTodoListQuery {}.execute(&self.runtime).await.unwrap();
        todo!()
    }
}
