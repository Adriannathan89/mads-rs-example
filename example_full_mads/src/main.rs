// src/main.rs
mod app;
mod schema;

use app::AppModule;
pub use schema::{post, user};
use mads::prelude::*;

#[mads::main]
async fn main() -> Result<(), HttpRuntimeError> {
    Mads::run::<AppModule>().await
}
