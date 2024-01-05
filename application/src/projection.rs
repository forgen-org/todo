use domain::todolist_event::TodoListEvent;
use framework::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct TodoListProjection {
    pub in_progress: HashMap<usize, String>,
    pub completed: HashMap<usize, String>,
}

impl Projection for TodoListProjection {
    type Event = TodoListEvent;

    fn apply(&mut self, events: &[Self::Event]) {
        for event in events {
            match event {
                TodoListEvent::TaskAdded(index, name) => {
                    self.in_progress
                        .insert((*index).into(), name.clone().into());
                }
                TodoListEvent::TaskCompleted(index) => {
                    self.completed.insert(
                        (*index).into(),
                        self.in_progress.remove(&(*index).into()).unwrap(),
                    );
                }
                TodoListEvent::TaskRemoved(index) => {
                    self.in_progress.remove(&(*index).into());
                }
            }
        }
    }
}
