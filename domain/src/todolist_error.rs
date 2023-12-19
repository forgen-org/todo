use framework::*;

#[derive(Debug, Error)]
pub enum TodoListError {
    #[error("Task name cannot be empty")]
    EmptyTaskName,
}
