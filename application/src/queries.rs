use crate::{ports::TodoListRepository, projections::TodoListProjection};
use framework::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTodoListQuery {}

#[async_trait]
impl<R> Query<R> for GetTodoListQuery
where
    R: TodoListRepository + Send + Sync,
{
    type Output = TodoListProjection;

    async fn execute(&self, services: &R) -> Result<Self::Output> {
        services.fetch().await
    }
}
