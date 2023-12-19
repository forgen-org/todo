use domain::todolist::{Completed, TodoList};
use framework::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct TodoListProjection {
    pub in_progress: Vec<Task>,
    pub completed: Vec<Task>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub index: usize,
}

impl Projection for TodoListProjection {
    type State = TodoList;

    fn project(state: &TodoList) -> Self {
        let mut in_progress = Vec::new();
        let mut completed = Vec::new();
        for (index, task) in state.tasks.iter().enumerate() {
            match task.completed {
                Completed::No => in_progress.push(Task {
                    name: task.name.clone(),
                    index,
                }),
                Completed::Yes => completed.push(Task {
                    name: task.name.clone(),
                    index,
                }),
            }
        }
        TodoListProjection {
            in_progress,
            completed,
        }
    }
}
