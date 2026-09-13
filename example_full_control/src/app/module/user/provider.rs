use std::sync::Arc;

use mads::prelude::*;
use sea_orm::DatabaseConnection;

use super::{
    r#trait::{UserRepository, UserService},
    repository::UserRepositoryImpl,
    service::UserServiceImpl,
};

#[provider]
pub fn user_repository(database: DatabaseConnection) -> Arc<dyn UserRepository + Send + Sync> {
    Arc::new(UserRepositoryImpl::new(database))
}

#[provider]
pub fn user_service(service: UserServiceImpl) -> Arc<dyn UserService + Send + Sync> {
    Arc::new(service)
}
