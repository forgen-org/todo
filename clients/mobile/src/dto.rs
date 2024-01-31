use application::projection::*;
use framework::*;

#[derive(uniffi::Record)]
pub struct TodoListDTO {
    pub tasks: Vec<TaskDTO>,
}

impl From<TodoList> for TodoListDTO {
    fn from(todo_list: TodoList) -> Self {
        Self {
            tasks: todo_list
                .tasks
                .into_iter()
                .map(|task| task.into())
                .collect(),
        }
    }
}

#[derive(uniffi::Record)]
pub struct TaskDTO {
    pub index: u32,
    pub name: String,
    pub status: TaskStatusDTO,
}

impl From<Task> for TaskDTO {
    fn from(task: Task) -> Self {
        Self {
            index: task.index as u32,
            name: task.name,
            status: task.status.into(),
        }
    }
}

#[derive(uniffi::Enum)]
pub enum TaskStatusDTO {
    Created,
    Completed,
}

impl From<TaskStatus> for TaskStatusDTO {
    fn from(task_status: TaskStatus) -> Self {
        match task_status {
            TaskStatus::Created => Self::Created,
            TaskStatus::Completed => Self::Completed,
        }
    }
}

#[derive(Debug, Error, uniffi::Error)]
pub enum ErrorDTO {
    #[error("{description}")]
    Error { description: String },
}

impl From<framework::AnyError> for ErrorDTO {
    fn from(error: framework::AnyError) -> Self {
        Self::Error {
            description: error.to_string(),
        }
    }
}
