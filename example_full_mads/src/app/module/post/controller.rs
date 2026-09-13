use std::sync::Arc;

use mads::prelude::*;

use super::{
    model::Post,
    r#trait::{CreatePostPayload, PostService, UpdatePostPayload},
    PostRoutes,
};

#[controller(routes = [PostRoutes])]
pub struct PostController {
    service: Arc<dyn PostService + Send + Sync>,
}

impl PostRoutes for PostController {
    async fn find(&self, Path(id): Path<i32>) -> HttpResult<Json<Post>> {
        self.service.find(id).await.into_http()?.map(Json)
            .ok_or_else(|| NotFound::new("post not found").into())
    }

    async fn find_by_user(&self, Path(user_id): Path<i32>) -> HttpResult<Json<Vec<Post>>> {
        self.service.find_by_user(user_id).await.into_http().map(Json)
    }

    async fn create(&self, Json(post): Json<CreatePostPayload>) -> HttpResult<Json<Post>> {
        self.service.create(post.title, post.body, post.user_id).await.into_http().map(Json)
    }

    async fn update(&self, Path(id): Path<i32>, Json(post): Json<UpdatePostPayload>) -> HttpResult<Json<Post>> {
        self.service.update(id, post.title, post.body).await.into_http()?.map(Json)
            .ok_or_else(|| NotFound::new("post not found").into())
    }
}
