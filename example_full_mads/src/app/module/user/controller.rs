use mads::prelude::*;
use std::sync::Arc;

use super::{
    UserRoutes,
    model::User,
    r#trait::{CreateUserPayload, UpdateUserNamePayload, UserService},
};

#[controller(routes = [UserRoutes])]
pub struct UserController {
    service: Arc<dyn UserService + Send + Sync>,
}

impl UserRoutes for UserController {
    async fn find(&self, Path(id): Path<i32>) -> HttpResult<Json<User>> {
        self.service
            .find(id)
            .await
            .into_http()?
            .map(Json)
            .ok_or_else(|| NotFound::new("user not found").into())
    }

    async fn find_by_email(&self, Path(email): Path<String>) -> HttpResult<Json<User>> {
        self.service
            .find_by_email(email)
            .await
            .into_http()?
            .map(Json)
            .ok_or_else(|| NotFound::new("user not found").into())
    }

    async fn create(&self, Json(user): Json<CreateUserPayload>) -> HttpResult<Json<User>> {
        self.service
            .create(user.email, user.name)
            .await
            .into_http()
            .map(Json)
    }

    async fn update_name(
        &self,
        Path(id): Path<i32>,
        Json(payload): Json<UpdateUserNamePayload>,
    ) -> HttpResult<Json<User>> {
        self.service
            .update_name(id, payload.name)
            .await
            .into_http()?
            .map(Json)
            .ok_or_else(|| NotFound::new("user not found").into())
    }
}
