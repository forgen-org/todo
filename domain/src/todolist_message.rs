use crate::todolist_scalar::{TaskIndex, TaskName};

pub enum TodoListMessage {
    AddTask(TaskName),
    RemoveTask(TaskIndex),
    CompleteTask(TaskIndex),
}
