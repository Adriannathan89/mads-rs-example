mod controller;
mod provider;
mod repository;
mod service;
mod r#trait;
mod model;
mod input;

pub use r#trait::*;

use mads::prelude::*;
use crate::app::provider::DatabaseModule;

#[module(imports = [DatabaseModule])]
pub struct UserModule;
