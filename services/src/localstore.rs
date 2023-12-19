use application::ports::*;
use framework::*;
use gloo_storage::{LocalStorage, Storage};

#[derive(Default)]
pub struct LocalStore {}

#[async_trait]
impl TodoListStore for LocalStore {
    async fn pull(&self) -> Result<TodoList> {
        let events: Vec<TodoListEvent> = LocalStorage::get("events").unwrap_or(vec![]);
        let mut state = TodoList::default();
        events.apply(&mut state);
        Ok(state)
    }

    async fn push(&self, events: &[TodoListEvent]) -> Result<()> {
        let mut current_events: Vec<TodoListEvent> = LocalStorage::get("events").unwrap_or(vec![]);
        current_events.extend_from_slice(events);
        LocalStorage::set("events", current_events)?;
        Ok(())
    }
}

#[async_trait]
impl TodoListRepository for LocalStore {
    async fn fetch(&self) -> Result<TodoListProjection> {
        Ok(LocalStorage::get("projection").unwrap_or_default())
    }

    async fn save(&self, projection: &TodoListProjection) -> Result<()> {
        LocalStorage::set("projection", projection)?;
        Ok(())
    }
}