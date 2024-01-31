use crate::projection::{Task, TaskStatus, TodoList};
use domain::todolist_event::TodoListEvent;
use framework::*;

/// We use the TodoList Projection as our Snapshot
/// This is a simple example, but in a real-world application
/// you would want to use a more efficient data structure
/// for your snapshot.
impl Snapshot<TodoListEvent> for TodoList {
    fn restore(&self) -> AnyResult<Vec<TodoListEvent>> {
        let mut events = Vec::new();

        for Task {
            index,
            name,
            status,
        } in &self.tasks
        {
            match status {
                TaskStatus::Created => {
                    events.push(TodoListEvent::TaskAdded(index.into(), name.try_into()?));
                }
                TaskStatus::Completed => {
                    events.push(TodoListEvent::TaskAdded(index.into(), name.try_into()?));
                    events.push(TodoListEvent::TaskCompleted(index.into()));
                }
            }
        }

        Ok(events)
    }
}
