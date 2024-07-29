use std::sync::Arc;

use crate::domain::{entity::user::{User}, repository::user_repository::UserRepository};

pub struct UserService {
    repository: Arc<dyn UserRepository>
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> UserService {
        UserService { repository }
    }

    pub async fn create_user(&self, user: &User) -> Result<(), sqlx::Error> {
        self.repository.save(&user).await?;
        Ok(())
    }

    pub async fn find_user_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        let user = self.repository.find_by_id(id).await?;
        Ok(user)
    }

    pub async fn delete_by_id(&self, id: i32) -> Result<(), sqlx::Error> {
        self.repository.delete(id).await?;
        Ok(())
    }

    pub async fn update(&self, user: &User) -> Result<(), sqlx::Error> {
        self.repository.update(&user).await?;
        Ok(())
    }
}

