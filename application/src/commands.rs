use crate::{
    ports::{TodoListRepository, TodoListStore},
    projections::TodoListProjection,
};
use domain::todolist_message::TodoListMessage;
use framework::*;
use serde::Deserialize;

// Shared command logic
async fn pull_and_push<R>(runtime: &R, message: &TodoListMessage) -> AnyResult<()>
where
    R: TodoListRepository + TodoListStore + Send + Sync,
{
    let mut todolist = runtime.pull().await?;
    let new_events = message.send(&todolist)?;
    new_events.apply(&mut todolist);
    let projection = TodoListProjection::project(&todolist);
    runtime.push(&new_events).await?;
    runtime.save(&projection).await?;
    Ok(())
}

#[derive(Deserialize)]
pub struct AddTaskCommand {
    pub name: String,
}

#[async_trait]
impl<R> Command<R> for AddTaskCommand
where
    R: TodoListRepository + TodoListStore + Send + Sync,
{
    // type Error = AnyError;

    async fn execute(&self, runtime: &R) -> AnyResult<()> {
        pull_and_push(runtime, &TodoListMessage::AddTask(self.name.clone())).await?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct RemoveTaskCommand {
    pub index: usize,
}

#[async_trait]
impl<R> Command<R> for RemoveTaskCommand
where
    R: TodoListRepository + TodoListStore + Send + Sync,
{
    async fn execute(&self, runtime: &R) -> Result<(), AnyError> {
        pull_and_push(runtime, &TodoListMessage::RemoveTask(self.index)).await?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct CompleteTaskCommand {
    pub index: usize,
}

#[async_trait]
impl<R> Command<R> for CompleteTaskCommand
where
    R: TodoListRepository + TodoListStore + Send + Sync,
{
    async fn execute(&self, runtime: &R) -> AnyResult<()> {
        pull_and_push(runtime, &TodoListMessage::CompleteTask(self.index)).await?;
        Ok(())
    }
}
