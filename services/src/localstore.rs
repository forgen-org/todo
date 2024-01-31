use application::port::*;
use framework::*;
use gloo_storage::{LocalStorage, Storage};

#[derive(Default)]
pub struct LocalStore {}

#[async_trait]
impl TodoListStore for LocalStore {
    async fn pull(&self) -> AnyResult<Vec<TodoListEvent>> {
        let events: Vec<TodoListEvent> = LocalStorage::get("events").unwrap_or(vec![]);
        Ok(events)
    }

    async fn push(&self, events: &[TodoListEvent]) -> AnyResult<()> {
        let mut current_events: Vec<TodoListEvent> = LocalStorage::get("events").unwrap_or(vec![]);
        current_events.extend_from_slice(events);
        LocalStorage::set("events", current_events)?;
        Ok(())
    }
}

#[async_trait]
impl TodoListRepository for LocalStore {
    async fn fetch(&self) -> AnyResult<TodoList> {
        Ok(LocalStorage::get("projection").unwrap_or_default())
    }

    async fn save(&self, projection: &TodoList) -> AnyResult<()> {
        LocalStorage::set("projection", projection)?;
        Ok(())
    }
}
