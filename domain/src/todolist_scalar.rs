use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub struct TaskIndex(pub usize);

impl From<usize> for TaskIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Into<usize> for TaskIndex {
    fn into(self) -> usize {
        self.0
    }
}

impl std::ops::AddAssign<usize> for TaskIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskName(pub String);

impl TryFrom<String> for TaskName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Task name cannot be empty".to_string());
        }

        Ok(Self(value))
    }
}

impl Into<String> for TaskName {
    fn into(self) -> String {
        self.0
    }
}
