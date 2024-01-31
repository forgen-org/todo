use framework::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct TaskIndex(pub usize);

impl From<&usize> for TaskIndex {
    fn from(value: &usize) -> Self {
        Self(*value)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskName(String);

#[derive(Debug, Error)]
pub enum TaskNameError {
    #[error("Task name cannot be empty")]
    TaskNameCannotBeEmpty,
}

impl TryFrom<&String> for TaskName {
    type Error = TaskNameError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(TaskNameError::TaskNameCannotBeEmpty);
        }

        Ok(Self(value.to_owned()))
    }
}

impl From<&TaskName> for String {
    fn from(value: &TaskName) -> Self {
        value.0.clone()
    }
}
