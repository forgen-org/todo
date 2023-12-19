use crate::todolist::{Completed, Task, TodoList};
use framework::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TodoListEvent {
    TaskAdded(String),
    TaskRemoved(usize),
    TaskCompleted(usize),
}

impl Events<TodoList> for Vec<TodoListEvent> {
    fn apply(&self, state: &mut TodoList) {
        for event in self {
            match event {
                TodoListEvent::TaskAdded(name) => {
                    state.tasks.push(Task {
                        name: name.clone(),
                        completed: Completed::No,
                    });
                }
                TodoListEvent::TaskRemoved(index) => {
                    state.tasks.remove(*index);
                }
                TodoListEvent::TaskCompleted(index) => {
                    state.tasks[*index].completed = Completed::Yes;
                }
            }
        }
    }
}
