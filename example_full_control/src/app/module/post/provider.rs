use std::sync::Arc;

use mads::prelude::*;
use sea_orm::DatabaseConnection;

use super::{
    r#trait::{PostRepository, PostService},
    repository::PostRepositoryImpl,
    service::PostServiceImpl,
};

#[provider]
pub fn post_repository(database: DatabaseConnection) -> Arc<dyn PostRepository + Send + Sync> {
    Arc::new(PostRepositoryImpl::new(database))
}

#[provider]
pub fn post_service(service: PostServiceImpl) -> Arc<dyn PostService + Send + Sync> {
    Arc::new(service)
}
