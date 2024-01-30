pub extern crate auto_delegate;
pub extern crate thiserror;

pub use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
pub use async_trait::async_trait;
pub use auto_delegate::*;
pub use thiserror::*;

pub trait Dispatch<Event, Error = AnyError> {
    fn dispatch(&self, events: &[Event]) -> Result<Vec<Event>, Error>;
}

pub trait Snapshot<Event, Error = AnyError>: Projection<Event> {
    fn restore(&self) -> Result<Vec<Event>, Error>;
}

pub trait Projection<Event> {
    fn apply(&mut self, events: &[Event]);
}

#[async_trait]
pub trait Execute<R, T = (), E = AnyError> {
    async fn execute(&self, runtime: &R) -> Result<T, E>;
}
