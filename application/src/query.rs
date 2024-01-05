use crate::{port::TodoListRepository, projection::TodoListProjection};
use framework::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTodoListQuery {}

#[async_trait]
impl<R> Execute<R> for GetTodoListQuery
where
    R: TodoListRepository + Send + Sync,
{
    type Output = TodoListProjection;

    async fn execute(&self, runtime: &R) -> AnyResult<TodoListProjection> {
        runtime.fetch().await
    }
}
