mod controller;
mod provider;
mod repository;
mod service;
mod r#trait;
mod model;
mod schema;

pub use r#trait::*;

use mads::prelude::*;

#[module]
pub struct UserModule;
