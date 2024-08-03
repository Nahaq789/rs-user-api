use std::sync::Arc;

use axum::async_trait;

use crate::domain::{entity::user::User, repository::user_repository::UserRepository};

use super::crypto_service::CryptoService;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn create_user(&self, user: User) -> Result<(), sqlx::Error>;
    async fn find_user_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error>;
    async fn delete_by_id(&self, id: i32) -> Result<(), sqlx::Error>;
    async fn update(&self, user: &User) -> Result<(), sqlx::Error>;
}

pub struct UserServiceImpl {
    repository: Arc<dyn UserRepository>,
    crypto: Arc<dyn CryptoService>,
}

impl UserServiceImpl {
    pub fn new(
        repository: Arc<dyn UserRepository>,
        crypto_service: Arc<dyn CryptoService>,
    ) -> UserServiceImpl {
        UserServiceImpl {
            repository,
            crypto: crypto_service,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(&self, mut user: User) -> Result<(), sqlx::Error> {
        let salt = self.crypto.create_salt();
        let hash_password = self.crypto.hash_password(user.password.as_bytes(), &salt);

        user.password = hash_password.unwrap();
        user.salt = salt;
        self.repository.save(&user).await?;
        Ok(())
    }

    async fn find_user_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        let user = self.repository.find_by_id(id).await?;
        Ok(user)
    }

    async fn delete_by_id(&self, id: i32) -> Result<(), sqlx::Error> {
        self.repository.delete(id).await?;
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), sqlx::Error> {
        self.repository.update(&user).await?;
        Ok(())
    }
}
