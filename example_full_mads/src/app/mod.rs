// src/app/mod.rs
mod module;

use mads::prelude::*;
use module::{post::PostModule, user::UserModule};

#[module(imports = [UserModule, PostModule])]
pub struct AppModule;
