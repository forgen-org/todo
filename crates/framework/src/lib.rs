#![feature(associated_type_defaults)]

pub extern crate auto_delegate;
pub extern crate thiserror;
use std::fmt::Display;

pub use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
pub use async_trait::async_trait;
pub use auto_delegate::*;
use serde::{Deserialize, Serialize};
pub use thiserror::*;

pub trait Message<S> {
    type Events: Events<S>;
    type Error: Display = AnyError;

    fn send(&self, state: &S) -> Result<Self::Events, Self::Error>;
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
    type Error: Display = AnyError;

    async fn execute(&self, runtime: &R) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait Query<R>: for<'de> Deserialize<'de> {
    type Output;
    type Error: Display = AnyError;

    async fn execute(&self, runtime: &R) -> Result<Self::Output, Self::Error>;
}
