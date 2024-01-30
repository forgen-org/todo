use crate::{todolist_event::TodoListEvent, todolist_scalar::TaskIndex};

pub struct TodoListSaga<'a>(pub &'a [TodoListEvent]);

pub(crate) enum TaskStatus {
    None,
    Created,
    Completed,
}

impl<'a> TodoListSaga<'a> {
    pub(crate) fn next_task_index(&self) -> TaskIndex {
        self.0
            .iter()
            .rev()
            .find(|event| matches!(event, TodoListEvent::TaskAdded(_, _)))
            .map(|event| match event {
                TodoListEvent::TaskAdded(index, _) => TaskIndex(index.0 + 1),
                _ => unreachable!(),
            })
            .unwrap_or(TaskIndex(0))
    }

    pub(crate) fn task_status(&self, index: TaskIndex) -> TaskStatus {
        let mut status = TaskStatus::None;

        for event in self.0 {
            match event {
                TodoListEvent::TaskAdded(candidate, _) => {
                    if &index == candidate {
                        status = TaskStatus::Created;
                    }
                }
                TodoListEvent::TaskCompleted(candidate) => {
                    if &index == candidate {
                        status = TaskStatus::Completed;
                    }
                }
                _ => {}
            }
        }

        status
    }
}
