use crate::port::{TodoListRepository, TodoListStore};
use domain::todolist_message::TodoListMessage;
use framework::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum Command {
    AddTask { name: String },
    RemoveTask { index: usize },
    CompleteTask { index: usize },
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Task name cannot be empty")]
    TaskNameCannotBeEmpty,
}

impl TryInto<TodoListMessage> for &Command {
    type Error = CommandError;

    fn try_into(self) -> Result<TodoListMessage, Self::Error> {
        Ok(match self {
            Command::AddTask { name } => TodoListMessage::AddTask(
                name.clone()
                    .try_into()
                    .map_err(|_| CommandError::TaskNameCannotBeEmpty)?,
            ),
            Command::RemoveTask { index } => TodoListMessage::RemoveTask((*index).into()),
            Command::CompleteTask { index } => TodoListMessage::CompleteTask((*index).into()),
        })
    }
}

#[async_trait]
impl<R> Execute<R> for Command
where
    R: TodoListRepository + TodoListStore + Send + Sync,
{
    async fn execute(&self, runtime: &R) -> AnyResult<()> {
        let message = self.try_into()?;

        // Pull the current state and apply the message
        let todolist = TodoListStore::pull(runtime).await?;
        let new_events = todolist.send(&message)?;
        TodoListStore::push(runtime, &new_events).await?;

        // Save the projection
        let mut projection = TodoListRepository::fetch(runtime).await?;
        projection.apply(&new_events);
        TodoListRepository::save(runtime, &projection).await?;

        Ok(())
    }
}
