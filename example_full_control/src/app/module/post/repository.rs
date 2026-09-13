use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use super::{
    model::{Post, entity},
    r#trait::{DatabaseResult, PostRepository},
};

pub struct PostRepositoryImpl {
    database: DatabaseConnection,
}

impl PostRepositoryImpl {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }
}

fn into_post(model: entity::Model) -> Post {
    Post {
        id: model.id,
        title: model.title,
        body: model.body,
        user_id: model.user_id,
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn create(&self, title: String, body: String, user_id: i32) -> DatabaseResult<Post> {
        entity::ActiveModel {
            title: Set(title),
            body: Set(body),
            user_id: Set(user_id),
            ..Default::default()
        }
        .insert(&self.database)
        .await
        .map(into_post)
    }

    async fn find(&self, id: i32) -> DatabaseResult<Option<Post>> {
        entity::Entity::find_by_id(id)
            .one(&self.database)
            .await
            .map(|model| model.map(into_post))
    }

    async fn find_by_user(&self, user_id: i32) -> DatabaseResult<Vec<Post>> {
        entity::Entity::find()
            .filter(entity::Column::UserId.eq(user_id))
            .all(&self.database)
            .await
            .map(|models| models.into_iter().map(into_post).collect())
    }

    async fn update(
        &self,
        id: i32,
        title: Option<String>,
        body: Option<String>,
    ) -> DatabaseResult<Option<Post>> {
        let Some(model) = entity::Entity::find_by_id(id).one(&self.database).await? else {
            return Ok(None);
        };

        let mut active: entity::ActiveModel = model.into();
        if let Some(title) = title {
            active.title = Set(title);
        }
        if let Some(body) = body {
            active.body = Set(body);
        }

        active.update(&self.database).await.map(into_post).map(Some)
    }
}
