use std::sync::Arc;

use mads::prelude::*;

use super::{
    r#trait::{UserRepository, UserService},
    repository::UserRepositoryImpl,
    service::UserServiceImpl,
};

#[provider]
pub fn user_repository(repo: UserRepositoryImpl) -> Arc<dyn UserRepository + Send + Sync> {
    Arc::new(repo)
}

#[provider]
pub fn user_service(service: UserServiceImpl) -> Arc<dyn UserService + Send + Sync> {
    Arc::new(service)
}
