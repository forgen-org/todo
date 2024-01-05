use framework::*;

use crate::todolist_scalar::TaskIndex;

#[derive(Debug, Error)]
pub enum TodoListError {
    #[error("Task already completed")]
    TaskAlreadyCompleted(TaskIndex),

    #[error("Task index out of bounds")]
    TaskNotFound(TaskIndex),
}
