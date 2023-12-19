use framework::*;

use crate::{todolist::TodoList, todolist_error::TodoListError, todolist_event::TodoListEvent};

pub enum TodoListMessage {
    AddTask(String),
    RemoveTask(usize),
    CompleteTask(usize),
}

impl Message<TodoList> for TodoListMessage {
    type Events = Vec<TodoListEvent>;
    fn send(&self, _state: &TodoList) -> Result<Self::Events> {
        let mut events = Vec::new();
        match self {
            TodoListMessage::AddTask(name) => {
                if name.is_empty() {
                    return Err(TodoListError::EmptyTaskName.into());
                }
                events.push(TodoListEvent::TaskAdded(name.clone()));
            }
            TodoListMessage::RemoveTask(index) => {
                events.push(TodoListEvent::TaskRemoved(*index));
            }
            TodoListMessage::CompleteTask(index) => {
                events.push(TodoListEvent::TaskCompleted(*index));
            }
        }
        Ok(events)
    }
}
