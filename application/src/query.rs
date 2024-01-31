use crate::{port::TodoListRepository, projection::TodoList};
use framework::*;
use serde::Deserialize;

#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[derive(Deserialize)]
pub struct GetTodoListQuery {}

#[async_trait]
impl<R> Execute<R, TodoList> for GetTodoListQuery
where
    R: TodoListRepository + Send + Sync,
{
    async fn execute(&self, runtime: &R) -> AnyResult<TodoList> {
        TodoListRepository::fetch(runtime).await
    }
}
