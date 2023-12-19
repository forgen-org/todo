pub use anyhow::{anyhow, Result};
pub use async_trait::async_trait;
pub use auto_delegate::*;
pub use thiserror::Error;
pub extern crate auto_delegate;

pub trait Message<S> {
    type Events: Events<S>;
    fn send(&self, state: &S) -> Result<Self::Events>;
}

pub trait Events<S> {
    fn apply(&self, state: &mut S);
}

pub trait Projection {
    type State;
    fn project(state: &Self::State) -> Self;
}

#[async_trait]
pub trait Command<R> {
    async fn execute(&self, services: &R) -> Result<()>;
}

#[async_trait]
pub trait Query<R> {
    type Output: Projection;
    async fn execute(&self, services: &R) -> Result<Self::Output>;
}
