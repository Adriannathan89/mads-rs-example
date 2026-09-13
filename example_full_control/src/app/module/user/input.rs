use mads::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Input, Deserialize, Serialize)]
pub struct CreateUserInput {
    #[validate(email, length(min = 1))]

    pub email: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}