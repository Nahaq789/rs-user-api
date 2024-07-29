use axum::async_trait;
use std::io::Error;

use crate::domain::entity::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, Error>;
    async fn delete(&self, id: i32) -> Result<(), Error>;
    async fn update(&self, user: &User) -> Result<(), Error>;
}
