use mads::prelude::*;
use std::sync::Arc;

use super::{
    UserRoutes,
    model::User,
    r#trait::{UpdateUserNamePayload, UserService},
    input::{CreateUserInput},
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
            .map_err(InternalError::new)?
            .map(Json)
            .ok_or_else(|| NotFound::new("user not found").into())
    }

    async fn find_by_email(&self, Path(email): Path<String>) -> HttpResult<Json<User>> {
        self.service
            .find_by_email(email)
            .await
            .map_err(InternalError::new)?
            .map(Json)
            .ok_or_else(|| NotFound::new("user not found").into())
    }

    async fn create(&self, body: ValidatedJson<CreateUserInput>) -> HttpResult<Json<User>> {
        let user = body.0;
        let user = self.service
            .create(user.email, user.name)
            .await
            .map_err(InternalError::new)?;
        Ok(Json(user))
    }

    async fn update_name(
        &self,
        Path(id): Path<i32>,
        Json(payload): Json<UpdateUserNamePayload>,
    ) -> HttpResult<Json<User>> {
        self.service
            .update_name(id, payload.name)
            .await
            .map_err(InternalError::new)?
            .map(Json)
            .ok_or_else(|| NotFound::new("user not found").into())
    }
}
