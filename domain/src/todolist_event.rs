use serde::{Deserialize, Serialize};

use crate::todolist_scalar::{TaskIndex, TaskName};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TodoListEvent {
    TaskAdded(TaskIndex, TaskName),
    TaskRemoved(TaskIndex),
    TaskCompleted(TaskIndex),
}
