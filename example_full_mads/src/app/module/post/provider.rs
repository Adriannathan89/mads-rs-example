use std::sync::Arc;

use mads::prelude::*;

use super::{
    r#trait::{PostRepository, PostService},
    repository::PostRepositoryImpl,
    service::PostServiceImpl,
};

#[provider]
pub fn post_repository(database: Database) -> Arc<dyn PostRepository + Send + Sync> {
    Arc::new(PostRepositoryImpl::new(database))
}

#[provider]
pub fn post_service(repository: Arc<dyn PostRepository + Send + Sync>) -> Arc<dyn PostService + Send + Sync> {
    Arc::new(PostServiceImpl::new(repository))
}
