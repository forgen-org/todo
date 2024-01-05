use std::collections::HashMap;

use crate::{
    todolist_error::TodoListError, todolist_event::TodoListEvent,
    todolist_message::TodoListMessage, todolist_scalar::TaskIndex,
};
use framework::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TodoList {
    next_index: TaskIndex,
    tasks: HashMap<TaskIndex, Completed>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Completed {
    Yes,
    No,
}

impl State for TodoList {
    type Error = TodoListError;
    type Event = TodoListEvent;
    type Message = TodoListMessage;

    fn apply(&mut self, events: &[Self::Event]) {
        for event in events {
            match event {
                TodoListEvent::TaskAdded(index, ..) => {
                    self.tasks.insert(*index, Completed::No);
                    self.next_index += 1;
                }
                TodoListEvent::TaskRemoved(index) => {
                    self.tasks.remove(index);
                }
                TodoListEvent::TaskCompleted(index) => {
                    self.tasks.insert(*index, Completed::Yes);
                }
            }
        }
    }

    fn send(&self, message: &Self::Message) -> Result<Vec<Self::Event>, Self::Error> {
        let mut events = Vec::new();
        match message {
            TodoListMessage::AddTask(name) => {
                events.push(TodoListEvent::TaskAdded(self.next_index, name.clone()));
            }
            TodoListMessage::RemoveTask(index) | TodoListMessage::CompleteTask(index) => {
                let task = self.tasks.get(index);

                match task {
                    None => return Err(TodoListError::TaskNotFound(*index)),
                    Some(Completed::Yes) => {
                        return Err(TodoListError::TaskAlreadyCompleted(*index))
                    }
                    Some(Completed::No) => match message {
                        TodoListMessage::RemoveTask(_) => {
                            events.push(TodoListEvent::TaskRemoved(*index));
                        }
                        TodoListMessage::CompleteTask(_) => {
                            events.push(TodoListEvent::TaskCompleted(*index));
                        }
                        _ => {}
                    },
                }
            }
        }
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use crate::todolist_scalar::TaskName;

    use super::*;

    #[test]
    fn task_index_addition() {
        let mut index = TaskIndex(5);
        index += 3;
        assert_eq!(index.0, 8);
    }

    #[test]
    fn task_name_creation() {
        let name = "Test Task".to_string();
        let task_name = TaskName::try_from(name.clone()).unwrap();
        assert_eq!(task_name.0, name);
    }

    #[test]
    fn task_name_creation_failure() {
        let name = "".to_string();
        assert!(TaskName::try_from(name).is_err());
    }

    #[test]
    fn add_task_to_todo_list() {
        let mut todo_list = TodoList::default();
        let task_name = "Buy milk".to_string();
        let message = TodoListMessage::AddTask(TaskName(task_name));
        let events = todo_list.send(&message).unwrap();
        todo_list.apply(&events);

        assert_eq!(todo_list.tasks.len(), 1);
        assert_eq!(todo_list.tasks[&TaskIndex(0)], Completed::No);
    }

    #[test]
    fn complete_task_in_todo_list() {
        let mut todo_list = TodoList::default();
        let task_name = "Buy milk".to_string();
        let add_message = TodoListMessage::AddTask(TaskName(task_name));
        let add_events = todo_list.send(&add_message).unwrap();
        todo_list.apply(&add_events);

        let complete_message = TodoListMessage::CompleteTask(TaskIndex(0));
        let complete_events = todo_list.send(&complete_message).unwrap();
        todo_list.apply(&complete_events);

        assert_eq!(todo_list.tasks[&TaskIndex(0)], Completed::Yes);
    }

    #[test]
    fn remove_task_from_todo_list() {
        let mut todo_list = TodoList::default();

        let task_name = "Buy milk".to_string();
        let add_message = TodoListMessage::AddTask(TaskName(task_name));
        let add_events = todo_list.send(&add_message).unwrap();
        todo_list.apply(&add_events);

        let remove_message = TodoListMessage::RemoveTask(TaskIndex(0));
        let remove_events = todo_list.send(&remove_message).unwrap();
        todo_list.apply(&remove_events);

        assert!(todo_list.tasks.is_empty());
    }

    #[test]
    fn error_on_completing_nonexistent_task() {
        let todo_list = TodoList::default();
        let message = TodoListMessage::CompleteTask(TaskIndex(10)); // Assuming no tasks have been added yet
        let result = todo_list.send(&message);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TodoListError::TaskNotFound(TaskIndex(10))
        );
    }

    #[test]
    fn error_on_completing_already_completed_task() {
        let mut todo_list = TodoList::default();
        let task_name = "Buy milk".to_string();
        let add_message = TodoListMessage::AddTask(TaskName(task_name));
        let add_events = todo_list.send(&add_message).unwrap();
        todo_list.apply(&add_events);

        let complete_message = TodoListMessage::CompleteTask(TaskIndex(0));
        let complete_events = todo_list.send(&complete_message).unwrap();
        todo_list.apply(&complete_events);

        let complete_again_message = TodoListMessage::CompleteTask(TaskIndex(0));
        let result = todo_list.send(&complete_again_message);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TodoListError::TaskAlreadyCompleted(TaskIndex(0))
        );
    }

    // Add more tests to cover additional edge cases and scenarios as needed
}
