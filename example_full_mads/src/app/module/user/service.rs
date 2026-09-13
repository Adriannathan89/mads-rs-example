use std::sync::Arc;

use mads::prelude::*;

use super::{model::User, r#trait::{UserRepository, UserService}};

#[service]
pub struct UserServiceImpl {
    repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserServiceImpl {
    pub async fn create(&self, email: String, name: String) -> DatabaseResult<User> {
        self.repository.create(email, name).await
    }

    pub async fn find(&self, id: i32) -> DatabaseResult<Option<User>> {
        self.repository.find(id).await
    }

    pub async fn find_by_email(&self, email: String) -> DatabaseResult<Option<User>> {
        self.repository.find_by_email(email).await
    }

    pub async fn update_name(&self, id: i32, name: String) -> DatabaseResult<Option<User>> {
        self.repository.update_name(id, name).await
    }
}

#[async_trait::async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, email: String, name: String) -> DatabaseResult<User> {
        UserServiceImpl::create(self, email, name).await
    }

    async fn find(&self, id: i32) -> DatabaseResult<Option<User>> {
        UserServiceImpl::find(self, id).await
    }

    async fn find_by_email(&self, email: String) -> DatabaseResult<Option<User>> {
        UserServiceImpl::find_by_email(self, email).await
    }

    async fn update_name(&self, id: i32, name: String) -> DatabaseResult<Option<User>> {
        UserServiceImpl::update_name(self, id, name).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;

    use super::*;

    #[derive(Debug, PartialEq)]
    enum RepositoryCall {
        Create { email: String, name: String },
        Find { id: i32 },
        FindByEmail { email: String },
        UpdateName { id: i32, name: String },
    }

    #[derive(Default)]
    struct MockUserRepository {
        calls: Mutex<Vec<RepositoryCall>>,
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, email: String, name: String) -> DatabaseResult<User> {
            self.calls.lock().unwrap().push(RepositoryCall::Create {
                email: email.clone(),
                name: name.clone(),
            });

            Ok(User { id: 1, name, email })
        }

        async fn find(&self, id: i32) -> DatabaseResult<Option<User>> {
            self.calls.lock().unwrap().push(RepositoryCall::Find { id });

            Ok(Some(User {
                id,
                name: "Ada Lovelace".to_owned(),
                email: "ada@example.com".to_owned(),
            }))
        }

        async fn find_by_email(&self, email: String) -> DatabaseResult<Option<User>> {
            self.calls.lock().unwrap().push(RepositoryCall::FindByEmail {
                email: email.clone(),
            });

            Ok(Some(User { id: 2, name: "Grace Hopper".to_owned(), email }))
        }

        async fn update_name(&self, id: i32, name: String) -> DatabaseResult<Option<User>> {
            self.calls.lock().unwrap().push(RepositoryCall::UpdateName {
                id,
                name: name.clone(),
            });

            Ok(Some(User {
                id,
                name,
                email: "ada@example.com".to_owned(),
            }))
        }
    }

    #[tokio::test]
    async fn delegates_every_operation_to_the_repository() {
        let repository = Arc::new(MockUserRepository::default());
        let service = UserServiceImpl(Arc::new(__MadsUserServiceImplInner {
            repository: repository.clone(),
        }));

        assert_eq!(
            service.create("ada@example.com".to_owned(), "Ada Lovelace".to_owned()).await.unwrap(),
            User {
                id: 1,
                name: "Ada Lovelace".to_owned(),
                email: "ada@example.com".to_owned(),
            },
        );

        assert_eq!(
            service.find(42).await.unwrap(),
            Some(User {
                id: 42,
                name: "Ada Lovelace".to_owned(),
                email: "ada@example.com".to_owned(),
            }),
        );

        assert_eq!(
            service.find_by_email("grace@example.com".to_owned()).await.unwrap(),
            Some(User {
                id: 2,
                name: "Grace Hopper".to_owned(),
                email: "grace@example.com".to_owned(),
            }),
        );

        assert_eq!(
            service.update_name(42, "Ada Byron".to_owned()).await.unwrap(),
            Some(User {
                id: 42,
                name: "Ada Byron".to_owned(),
                email: "ada@example.com".to_owned(),
            }),
        );

        assert_eq!(
            *repository.calls.lock().unwrap(),
            vec![
                RepositoryCall::Create {
                    email: "ada@example.com".to_owned(),
                    name: "Ada Lovelace".to_owned(),
                },
                RepositoryCall::Find { id: 42 },
                RepositoryCall::FindByEmail {
                    email: "grace@example.com".to_owned(),
                },
                RepositoryCall::UpdateName {
                    id: 42,
                    name: "Ada Byron".to_owned(),
                },
            ],
        );
    }
}
