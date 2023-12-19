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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todolist::TodoList;
    // use crate::todolist_error::TodoListError;

    #[test]
    fn test_add_task() {
        let todo_list = TodoList::default();
        let message = TodoListMessage::AddTask("New Task".to_string());

        let events = message.send(&todo_list).unwrap();

        assert_eq!(events.len(), 1);
        match &events[0] {
            TodoListEvent::TaskAdded(name) => assert_eq!(name, "New Task"),
            _ => panic!("Expected TaskAdded event"),
        }
    }

    // TODO: maybe we don't use anyhow because it offuscates the error type
    // #[test]
    // fn test_add_empty_task() {
    //     let todo_list = TodoList::default();
    //     let message = TodoListMessage::AddTask("".to_string());

    //     let result = message.send(&todo_list);

    //     assert!(matches!(result, anyhow!(TodoListError::EmptyTaskName)));
    // }

    #[test]
    fn test_remove_task() {
        let todo_list = TodoList::default();
        let message = TodoListMessage::RemoveTask(0);

        let events = message.send(&todo_list).unwrap();

        assert_eq!(events.len(), 1);
        match &events[0] {
            TodoListEvent::TaskRemoved(index) => assert_eq!(*index, 0),
            _ => panic!("Expected TaskRemoved event"),
        }
    }

    #[test]
    fn test_complete_task() {
        let todo_list = TodoList::default();
        let message = TodoListMessage::CompleteTask(0);

        let events = message.send(&todo_list).unwrap();

        assert_eq!(events.len(), 1);
        match &events[0] {
            TodoListEvent::TaskCompleted(index) => assert_eq!(*index, 0),
            _ => panic!("Expected TaskCompleted event"),
        }
    }
}
