use crate::{
    todolist_error::TodoListError,
    todolist_event::TodoListEvent,
    todolist_saga::{TaskStatus, TodoListSaga},
    todolist_scalar::{TaskIndex, TaskName},
};
use framework::*;

pub enum TodoListMessage {
    AddTask(TaskName),
    RemoveTask(TaskIndex),
    CompleteTask(TaskIndex),
}

impl Dispatch<TodoListEvent, TodoListError> for TodoListMessage {
    fn dispatch(&self, events: &[TodoListEvent]) -> Result<Vec<TodoListEvent>, TodoListError> {
        let saga = TodoListSaga(events);
        let mut events = Vec::new();

        match self {
            TodoListMessage::AddTask(name) => {
                events.push(TodoListEvent::TaskAdded(
                    saga.next_task_index(),
                    name.clone(),
                ));
            }
            TodoListMessage::CompleteTask(index) => match saga.task_status(*index) {
                TaskStatus::None => return Err(TodoListError::TaskNotFound(*index)),
                TaskStatus::Completed => return Err(TodoListError::TaskAlreadyCompleted(*index)),
                TaskStatus::Created => {
                    events.push(TodoListEvent::TaskCompleted(*index));
                }
            },
            TodoListMessage::RemoveTask(index) => match saga.task_status(*index) {
                TaskStatus::None => return Err(TodoListError::TaskNotFound(*index)),
                TaskStatus::Completed => return Err(TodoListError::TaskAlreadyCompleted(*index)),
                TaskStatus::Created => {
                    events.push(TodoListEvent::TaskRemoved(*index));
                }
            },
        }

        Ok(events)
    }
}
