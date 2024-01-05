pub use crate::projection::TodoListProjection;
pub use domain::{todolist_event::TodoListEvent, todolist_state::TodoList};
use framework::*;

#[async_trait]
#[delegate]
pub trait TodoListStore {
    async fn pull(&self) -> AnyResult<TodoList>;
    async fn push(&self, events: &[TodoListEvent]) -> AnyResult<()>;
}

#[async_trait]
#[delegate]
pub trait TodoListRepository {
    async fn fetch(&self) -> AnyResult<TodoListProjection>;
    async fn save(&self, projection: &TodoListProjection) -> AnyResult<()>;
}
