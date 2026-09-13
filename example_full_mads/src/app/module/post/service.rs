use std::sync::Arc;

use async_trait::async_trait;
use mads::prelude::*;

use super::{model::Post, r#trait::{PostRepository, PostService}};

pub struct PostServiceImpl {
    repository: Arc<dyn PostRepository + Send + Sync>,
}

impl PostServiceImpl {
    pub(super) fn new(repository: Arc<dyn PostRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl PostService for PostServiceImpl {
    async fn create(&self, title: String, body: String, user_id: i32) -> DatabaseResult<Post> {
        self.repository.create(title, body, user_id).await
    }

    async fn find(&self, id: i32) -> DatabaseResult<Option<Post>> {
        self.repository.find(id).await
    }

    async fn find_by_user(&self, user_id: i32) -> DatabaseResult<Vec<Post>> {
        self.repository.find_by_user(user_id).await
    }

    async fn update(&self, id: i32, title: Option<String>, body: Option<String>) -> DatabaseResult<Option<Post>> {
        self.repository.update(id, title, body).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use super::*;

    #[derive(Debug, PartialEq)]
    enum RepositoryCall {
        Create { title: String, body: String, user_id: i32 },
        Find { id: i32 },
        FindByUser { user_id: i32 },
        Update { id: i32, title: Option<String>, body: Option<String> },
    }

    #[derive(Default)]
    struct MockPostRepository {
        calls: Mutex<Vec<RepositoryCall>>,
    }

    #[async_trait]
    impl PostRepository for MockPostRepository {
        async fn create(&self, title: String, body: String, user_id: i32) -> DatabaseResult<Post> {
            self.calls.lock().unwrap().push(RepositoryCall::Create {
                title: title.clone(),
                body: body.clone(),
                user_id,
            });

            Ok(Post { id: 1, title, body, user_id })
        }

        async fn find(&self, id: i32) -> DatabaseResult<Option<Post>> {
            self.calls.lock().unwrap().push(RepositoryCall::Find { id });

            Ok(Some(Post { id, title: "title".to_owned(), body: "body".to_owned(), user_id: 7 }))
        }

        async fn find_by_user(&self, user_id: i32) -> DatabaseResult<Vec<Post>> {
            self.calls.lock().unwrap().push(RepositoryCall::FindByUser { user_id });

            Ok(vec![Post { id: 2, title: "title".to_owned(), body: "body".to_owned(), user_id }])
        }

        async fn update(&self, id: i32, title: Option<String>, body: Option<String>) -> DatabaseResult<Option<Post>> {
            self.calls.lock().unwrap().push(RepositoryCall::Update {
                id,
                title: title.clone(),
                body: body.clone(),
            });

            Ok(Some(Post { id, title: title.unwrap_or_default(), body: body.unwrap_or_default(), user_id: 7 }))
        }
    }

    #[tokio::test]
    async fn delegates_every_operation_to_the_repository() {
        let repository = Arc::new(MockPostRepository::default());
        let service = PostServiceImpl::new(repository.clone());

        assert_eq!(
            service.create("First post".to_owned(), "Content".to_owned(), 7).await.unwrap(),
            Post { id: 1, title: "First post".to_owned(), body: "Content".to_owned(), user_id: 7 },
        );

        assert_eq!(
            service.find(42).await.unwrap(),
            Some(Post { id: 42, title: "title".to_owned(), body: "body".to_owned(), user_id: 7 }),
        );

        assert_eq!(
            service.find_by_user(7).await.unwrap(),
            vec![Post { id: 2, title: "title".to_owned(), body: "body".to_owned(), user_id: 7 }],
        );

        assert_eq!(
            service.update(2, Some("Updated title".to_owned()), None).await.unwrap(),
            Some(Post { id: 2, title: "Updated title".to_owned(), body: String::new(), user_id: 7 }),
        );

        assert_eq!(
            *repository.calls.lock().unwrap(),
            vec![
                RepositoryCall::Create {
                    title: "First post".to_owned(),
                    body: "Content".to_owned(),
                    user_id: 7,
                },
                RepositoryCall::Find { id: 42 },
                RepositoryCall::FindByUser { user_id: 7 },
                RepositoryCall::Update {
                    id: 2,
                    title: Some("Updated title".to_owned()),
                    body: None,
                },
            ],
        );
    }
}
