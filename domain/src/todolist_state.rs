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
            TodoListMessage::RemoveTask(index) => {
                if !self.tasks.contains_key(index) {
                    return Err(TodoListError::TaskNotFound(*index));
                }
                events.push(TodoListEvent::TaskRemoved(*index));
            }
            TodoListMessage::CompleteTask(index) => {
                let task = self.tasks.get(index);

                match task {
                    None => return Err(TodoListError::TaskNotFound(*index)),
                    Some(Completed::Yes) => {
                        return Err(TodoListError::TaskAlreadyCompleted(*index))
                    }
                    Some(Completed::No) => {
                        events.push(TodoListEvent::TaskCompleted(*index));
                    }
                }
            }
        }
        Ok(events)
    }
}
