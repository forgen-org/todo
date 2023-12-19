use crate::{projections::TodoListProjection, services::TodoListRepository};
use framework::*;

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
