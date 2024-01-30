use application::port::*;
use framework::*;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct MemStore {
    events: Mutex<Vec<TodoListEvent>>,
    projection: Mutex<TodoList>,
}

#[async_trait]
impl TodoListStore for MemStore {
    async fn pull(&self) -> AnyResult<Vec<TodoListEvent>> {
        let events = self.events.lock().await.clone();
        Ok(events)
    }

    async fn push(&self, new_events: &[TodoListEvent]) -> AnyResult<()> {
        self.events.lock().await.extend_from_slice(new_events);
        Ok(())
    }
}

#[async_trait]
impl TodoListRepository for MemStore {
    async fn fetch(&self) -> AnyResult<TodoList> {
        Ok(self.projection.lock().await.to_owned())
    }

    async fn save(&self, projection: &TodoList) -> AnyResult<()> {
        let mut current_projection = self.projection.lock().await;
        *current_projection = projection.clone();
        Ok(())
    }
}
