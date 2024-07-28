use std::io::Error;
use axum::async_trait;

use crate::domain::entity::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), Error>;
    async fn find_by_id(&self,id: u32) -> Result<Option<User>, Error>;
}