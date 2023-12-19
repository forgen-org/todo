use application::ports::*;
use framework::*;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct MemStore {
    events: Mutex<Vec<TodoListEvent>>,
    projection: Mutex<TodoListProjection>,
}

#[async_trait]
impl TodoListStore for MemStore {
    async fn pull(&self) -> Result<TodoList> {
        let mut todolist = TodoList::default();
        self.events.lock().await.apply(&mut todolist);
        Ok(todolist)
    }

    async fn push(&self, new_events: &[TodoListEvent]) -> Result<()> {
        self.events.lock().await.extend_from_slice(new_events);
        Ok(())
    }
}

#[async_trait]
impl TodoListRepository for MemStore {
    async fn fetch(&self) -> Result<TodoListProjection> {
        Ok(self.projection.lock().await.to_owned())
    }

    async fn save(&self, projection: &TodoListProjection) -> Result<()> {
        let mut current_projection = self.projection.lock().await;
        *current_projection = projection.clone();
        Ok(())
    }
}
