use std::sync::Arc;

use crate::domain::{entity::user::User, repository::user_repository::UserRepository};

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
}

