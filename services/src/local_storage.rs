use crate::Services;
use application::services::*;
use framework::*;
use gloo_storage::{LocalStorage, Storage};

#[async_trait]
impl TodoListStore for Services {
    async fn pull(&self) -> Result<TodoList> {
        let events: Vec<TodoListEvent> = LocalStorage::get("events").unwrap_or(vec![]);
        let mut state = TodoList::default();
        events.apply(&mut state);
        Ok(state)
    }

    async fn push(&self, new_events: &[TodoListEvent]) -> Result<()> {
        let mut events: Vec<TodoListEvent> = LocalStorage::get("events").unwrap_or(vec![]);
        events.extend_from_slice(&new_events);
        LocalStorage::set("events", events)?;
        Ok(())
    }
}

#[async_trait]
impl TodoListRepository for Services {
    async fn fetch(&self) -> Result<TodoListProjection> {
        Ok(LocalStorage::get("projection").unwrap_or_default())
    }

    async fn save(&self, projection: &TodoListProjection) -> Result<()> {
        LocalStorage::set("projection", projection)?;
        Ok(())
    }
}
