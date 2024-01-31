pub use crate::projection::TodoList;
pub use domain::todolist_event::TodoListEvent;
use framework::*;

#[async_trait]
#[delegate]
pub trait TodoListStore {
    async fn pull(&self) -> AnyResult<Vec<TodoListEvent>>;
    async fn push(&self, events: &[TodoListEvent]) -> AnyResult<()>;
}

#[async_trait]
#[delegate]
pub trait TodoListRepository {
    async fn fetch(&self) -> AnyResult<TodoList>;
    async fn save(&self, projection: &TodoList) -> AnyResult<()>;
}
