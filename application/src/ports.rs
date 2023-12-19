pub use crate::projections::TodoListProjection;
pub use domain::{todolist::TodoList, todolist_event::TodoListEvent};
use framework::*;

#[async_trait]
#[delegate]
pub trait TodoListStore {
    async fn pull(&self) -> Result<TodoList>;
    async fn push(&self, events: &[TodoListEvent]) -> Result<()>;
}

#[async_trait]
#[delegate]
pub trait TodoListRepository {
    async fn fetch(&self) -> Result<TodoListProjection>;
    async fn save(&self, projection: &TodoListProjection) -> Result<()>;
}
