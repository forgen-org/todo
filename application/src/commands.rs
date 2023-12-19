use crate::{
    projections::TodoListProjection,
    services::{TodoListRepository, TodoListStore},
};
use domain::todolist_message::TodoListMessage;
use framework::*;

// Shared command logic
async fn pull_and_push<R>(services: &R, message: &TodoListMessage) -> Result<()>
where
    R: TodoListRepository + TodoListStore + Send + Sync,
{
    let mut todolist = services.pull().await?;
    let new_events = message.send(&todolist)?;
    new_events.apply(&mut todolist);
    let projection = TodoListProjection::project(&todolist);
    services.push(&new_events).await?;
    services.save(&projection).await?;
    Ok(())
}

pub struct AddTaskCommand {
    pub name: String,
}

#[async_trait]
impl<R> Command<R> for AddTaskCommand
where
    R: TodoListRepository + TodoListStore + Send + Sync,
{
    async fn execute(&self, services: &R) -> Result<()> {
        pull_and_push(services, &TodoListMessage::AddTask(self.name.clone())).await?;
        Ok(())
    }
}

pub struct RemoveTaskCommand {
    pub index: usize,
}

#[async_trait]
impl<R> Command<R> for RemoveTaskCommand
where
    R: TodoListRepository + TodoListStore + Send + Sync,
{
    async fn execute(&self, services: &R) -> Result<()> {
        pull_and_push(services, &TodoListMessage::RemoveTask(self.index)).await?;
        Ok(())
    }
}

pub struct CompleteTaskCommand {
    pub index: usize,
}

#[async_trait]
impl<R> Command<R> for CompleteTaskCommand
where
    R: TodoListRepository + TodoListStore + Send + Sync,
{
    async fn execute(&self, services: &R) -> Result<()> {
        pull_and_push(services, &TodoListMessage::CompleteTask(self.index)).await?;
        Ok(())
    }
}
