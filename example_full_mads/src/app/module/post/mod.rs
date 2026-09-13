mod controller;
mod model;
mod provider;
mod repository;
mod schema;
mod service;
mod r#trait;

pub use r#trait::*;

use mads::prelude::*;

#[module]
pub struct PostModule;
