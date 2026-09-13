use async_trait::async_trait;
use mads::prelude::*;

use super::model::User;
use super::input::{CreateUserInput};

pub type DatabaseResult<T> = Result<T, sea_orm::DbErr>;

#[derive(serde::Deserialize)]
pub struct UpdateUserNamePayload {
    pub name: String,
}

#[routes(prefix = "/api")]
pub trait UserRoutes {
    #[get("/users/:id")]
    async fn find(&self, id: Path<i32>) -> HttpResult<Json<User>>;

    #[get("/users/by-email/:email")]
    async fn find_by_email(&self, email: Path<String>) -> HttpResult<Json<User>>;

    #[post("/users")]
    async fn create(&self, body: ValidatedJson<CreateUserInput>) -> HttpResult<Json<User>>;

    #[put("/users/:id")]
    async fn update_name(
        &self,
        id: Path<i32>,
        payload: Json<UpdateUserNamePayload>,
    ) -> HttpResult<Json<User>>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, email: String, name: String) -> DatabaseResult<User>;
    async fn find(&self, id: i32) -> DatabaseResult<Option<User>>;
    async fn find_by_email(&self, email: String) -> DatabaseResult<Option<User>>;
    async fn update_name(&self, id: i32, name: String) -> DatabaseResult<Option<User>>;
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, email: String, name: String) -> DatabaseResult<User>;
    async fn find(&self, id: i32) -> DatabaseResult<Option<User>>;
    async fn find_by_email(&self, email: String) -> DatabaseResult<Option<User>>;
    async fn update_name(&self, id: i32, name: String) -> DatabaseResult<Option<User>>;
}
