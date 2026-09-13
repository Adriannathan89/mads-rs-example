use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use super::{
    model::{User, entity},
    r#trait::{DatabaseResult, UserRepository},
};

pub struct UserRepositoryImpl {
    database: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }
}

fn into_user(model: entity::Model) -> User {
    User {
        id: model.id,
        name: model.name,
        email: model.email,
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, email: String, name: String) -> DatabaseResult<User> {
        entity::ActiveModel {
            email: Set(email),
            name: Set(name),
            ..Default::default()
        }
        .insert(&self.database)
        .await
        .map(into_user)
    }

    async fn find(&self, id: i32) -> DatabaseResult<Option<User>> {
        entity::Entity::find_by_id(id)
            .one(&self.database)
            .await
            .map(|model| model.map(into_user))
    }

    async fn find_by_email(&self, email: String) -> DatabaseResult<Option<User>> {
        entity::Entity::find()
            .filter(entity::Column::Email.eq(email))
            .one(&self.database)
            .await
            .map(|model| model.map(into_user))
    }

    async fn update_name(&self, id: i32, name: String) -> DatabaseResult<Option<User>> {
        let Some(model) = entity::Entity::find_by_id(id).one(&self.database).await? else {
            return Ok(None);
        };

        let mut active: entity::ActiveModel = model.into();
        active.name = Set(name);
        active.update(&self.database).await.map(into_user).map(Some)
    }
}
