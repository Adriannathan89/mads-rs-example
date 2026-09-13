use async_trait::async_trait;
use mads::prelude::*;

use super::model::Post;

#[derive(serde::Deserialize)]
pub struct CreatePostPayload {
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[derive(serde::Deserialize)]
pub struct UpdatePostPayload {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[routes(prefix = "/api")]
pub trait PostRoutes {
    #[get("/posts/:id")]
    async fn find(&self, id: Path<i32>) -> HttpResult<Json<Post>>;

    #[get("/posts/by-user/:user_id")]
    async fn find_by_user(&self, user_id: Path<i32>) -> HttpResult<Json<Vec<Post>>>;

    #[post("/posts")]
    async fn create(&self, post: Json<CreatePostPayload>) -> HttpResult<Json<Post>>;

    #[put("/posts/:id")]
    async fn update(&self, id: Path<i32>, post: Json<UpdatePostPayload>) -> HttpResult<Json<Post>>;
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create(&self, title: String, body: String, user_id: i32) -> DatabaseResult<Post>;
    async fn find(&self, id: i32) -> DatabaseResult<Option<Post>>;
    async fn find_by_user(&self, user_id: i32) -> DatabaseResult<Vec<Post>>;
    async fn update(&self, id: i32, title: Option<String>, body: Option<String>) -> DatabaseResult<Option<Post>>;
}

#[async_trait]
pub trait PostService: Send + Sync {
    async fn create(&self, title: String, body: String, user_id: i32) -> DatabaseResult<Post>;
    async fn find(&self, id: i32) -> DatabaseResult<Option<Post>>;
    async fn find_by_user(&self, user_id: i32) -> DatabaseResult<Vec<Post>>;
    async fn update(&self, id: i32, title: Option<String>, body: Option<String>) -> DatabaseResult<Option<Post>>;
}
