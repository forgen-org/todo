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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_addition() {
        let mut todo_list = TodoList::default();
        let events = vec![TodoListEvent::TaskAdded("Test Task".to_string())];

        events.apply(&mut todo_list);

        assert_eq!(todo_list.tasks.len(), 1);
        assert_eq!(todo_list.tasks[0].name, "Test Task");
        assert_eq!(todo_list.tasks[0].completed, Completed::No);
    }

    #[test]
    fn test_task_removal() {
        let mut todo_list = TodoList {
            tasks: vec![Task {
                name: "Test Task".to_string(),
                completed: Completed::No,
            }],
        };
        let events = vec![TodoListEvent::TaskRemoved(0)];

        events.apply(&mut todo_list);

        assert!(todo_list.tasks.is_empty());
    }

    #[test]
    fn test_task_completion() {
        let mut todo_list = TodoList {
            tasks: vec![Task {
                name: "Test Task".to_string(),
                completed: Completed::No,
            }],
        };
        let events = vec![TodoListEvent::TaskCompleted(0)];

        events.apply(&mut todo_list);

        assert_eq!(todo_list.tasks[0].completed, Completed::Yes);
    }

    // Additional tests for error handling can be added here
}
