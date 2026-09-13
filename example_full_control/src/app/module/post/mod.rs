mod controller;
mod model;
mod provider;
mod repository;
mod service;
mod r#trait;

pub use r#trait::*;

use mads::prelude::*;
use crate::app::provider::DatabaseModule;

#[module(imports = [DatabaseModule])]
pub struct PostModule;
