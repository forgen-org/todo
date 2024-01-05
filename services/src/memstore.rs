use application::port::*;
use framework::*;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct MemStore {
    events: Mutex<Vec<TodoListEvent>>,
    projection: Mutex<TodoListProjection>,
}

#[async_trait]
impl TodoListStore for MemStore {
    async fn pull(&self) -> AnyResult<TodoList> {
        let mut todolist = TodoList::default();
        todolist.apply(&self.events.lock().await);
        Ok(todolist)
    }

    async fn push(&self, new_events: &[TodoListEvent]) -> AnyResult<()> {
        self.events.lock().await.extend_from_slice(new_events);
        Ok(())
    }
}

#[async_trait]
impl TodoListRepository for MemStore {
    async fn fetch(&self) -> AnyResult<TodoListProjection> {
        Ok(self.projection.lock().await.to_owned())
    }

    async fn save(&self, projection: &TodoListProjection) -> AnyResult<()> {
        let mut current_projection = self.projection.lock().await;
        *current_projection = projection.clone();
        Ok(())
    }
}
