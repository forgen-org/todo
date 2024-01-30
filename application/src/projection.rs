use domain::todolist_event::TodoListEvent;
use framework::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub index: usize,
    pub name: String,
    pub status: TaskStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Created,
    Completed,
}

impl Projection<TodoListEvent> for TodoList {
    fn apply(&mut self, events: &[TodoListEvent]) {
        for event in events {
            match event {
                TodoListEvent::TaskAdded(index, name) => {
                    self.tasks.push(Task {
                        index: index.0,
                        name: name.into(),
                        status: TaskStatus::Created,
                    });
                }
                TodoListEvent::TaskCompleted(index) => {
                    let task = self
                        .tasks
                        .iter_mut()
                        .find(|task| task.index == index.0)
                        .unwrap();
                    task.status = TaskStatus::Completed;
                }
                TodoListEvent::TaskRemoved(index) => {
                    self.tasks.retain(|task| task.index != index.0);
                }
            }
        }
    }
}
