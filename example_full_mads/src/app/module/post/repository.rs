use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use mads::prelude::*;

use super::{
    model::{NewPost, Post, PostChangeset},
    r#trait::PostRepository,
    schema::posts,
};

pub struct PostRepositoryImpl {
    database: Database,
}

impl PostRepositoryImpl {
    pub(super) fn new(database: Database) -> Self {
        Self { database }
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn create(&self, title: String, body: String, user_id: i32) -> DatabaseResult<Post> {
        self.database
            .run(move |connection| {
                diesel::insert_into(posts::table)
                    .values(NewPost { title, body, user_id })
                    .returning(Post::as_returning())
                    .get_result(connection)
            })
            .await
    }

    async fn find(&self, id: i32) -> DatabaseResult<Option<Post>> {
        self.database
            .run(move |connection| {
                posts::table
                    .find(id)
                    .select(Post::as_select())
                    .first(connection)
                    .optional()
            })
            .await
    }

    async fn find_by_user(&self, user_id: i32) -> DatabaseResult<Vec<Post>> {
        self.database
            .run(move |connection| {
                posts::table
                    .filter(posts::user_id.eq(user_id))
                    .select(Post::as_select())
                    .load(connection)
            })
            .await
    }

    async fn update(
        &self,
        id: i32,
        title: Option<String>,
        body: Option<String>,
    ) -> DatabaseResult<Option<Post>> {
        self.database
            .run(move |connection| {
                diesel::update(posts::table.find(id))
                    .set(PostChangeset { title, body })
                    .returning(Post::as_returning())
                    .get_result(connection)
                    .optional()
            })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mads::core::{ConfigBuilder, DotenvSource, TomlSource};

    #[tokio::test]
    async fn find_returns_none_for_an_unknown_post() {
        let config = ConfigBuilder::new()
            .dotenv(DotenvSource::optional(".env"))
            .source(TomlSource::file("mads.toml"))
            .build()
            .expect("project configuration should load");
        let database = Database::from_config(
            &DatabaseConfig::from_config(&config)
                .expect("database configuration should load"),
        )
        .expect("database pool should build");
        let repository = PostRepositoryImpl::new(database);

        assert_eq!(repository.find(i32::MIN).await.unwrap(), None);
    }
}
