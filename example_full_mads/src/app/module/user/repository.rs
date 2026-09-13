use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use mads::prelude::*;

use super::{model::{NewUser, User}, schema::users};
use super::r#trait::UserRepository;

#[repository]
pub struct UserRepositoryImpl {
    database: Database,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, email: String, name: String) -> DatabaseResult<User> {
        let new_user = NewUser {
            email,
            name,
        };
        self.database
            .run(move |conn| {
                diesel::insert_into(users::table)
                    .values(new_user)
                    .returning(User::as_returning())
                    .get_result(conn)
            })
            .await
    }

    async fn find(&self, id: i32) -> DatabaseResult<Option<User>> {
        self.database
            .run(move |conn| {
                users::table
                    .find(id)
                    .select(User::as_select())
                    .first(conn)
                    .optional()
            })
            .await
    }

    async fn find_by_email(&self, email: String) -> DatabaseResult<Option<User>> {
        self.database
            .run(move |conn| {
                users::table
                    .filter(users::email.eq(email))
                    .select(User::as_select())
                    .first(conn)
                    .optional()
            })
            .await
    }

    async fn update_name(&self, id: i32, name: String) -> DatabaseResult<Option<User>> {
        self.database
            .run(move |conn| {
                diesel::update(users::table.find(id))
                    .set(users::name.eq(name))
                    .returning(User::as_returning())
                    .get_result(conn)
                    .optional()
            })
            .await
    }
}
