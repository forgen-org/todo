#![feature(associated_type_defaults)]

pub extern crate auto_delegate;
pub extern crate thiserror;
use std::fmt::Display;

pub use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
pub use async_trait::async_trait;
pub use auto_delegate::*;
use serde::{Deserialize, Serialize};
pub use thiserror::*;

pub trait State: Default + Serialize + for<'de> Deserialize<'de> {
    type Error: Display = AnyError;
    type Event: Serialize + for<'de> Deserialize<'de>;
    type Message;

    fn apply(&mut self, events: &[Self::Event]);
    fn send(&self, message: &Self::Message) -> Result<Vec<Self::Event>, Self::Error>;
}

pub trait Projection: Default + Serialize + for<'de> Deserialize<'de> {
    type Event: Serialize + for<'de> Deserialize<'de>;

    fn apply(&mut self, events: &[Self::Event]);
}

#[async_trait]
pub trait Execute<R>: for<'de> Deserialize<'de> {
    type Output = ();
    type Error: Display = AnyError;

    async fn execute(&self, runtime: &R) -> Result<Self::Output, Self::Error>;
}
