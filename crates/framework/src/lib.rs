pub extern crate auto_delegate;
pub use anyhow::{anyhow, Result};
pub use async_trait::async_trait;
pub use auto_delegate::*;
use serde::{Deserialize, Serialize};
pub use thiserror::Error;

pub trait Message<S> {
    type Events: Events<S>;
    fn send(&self, state: &S) -> Result<Self::Events>;
}

pub trait Events<S>: Serialize + for<'de> Deserialize<'de> {
    fn apply(&self, state: &mut S);
}

pub trait Projection: Serialize + for<'de> Deserialize<'de> {
    type State;
    fn project(state: &Self::State) -> Self;
}

#[async_trait]
pub trait Command<R>: for<'de> Deserialize<'de> {
    async fn execute(&self, runtime: &R) -> Result<()>;
}

#[async_trait]
pub trait Query<R>: for<'de> Deserialize<'de> {
    type Output;
    async fn execute(&self, runtime: &R) -> Result<Self::Output>;
}
