use std::io::Error;
use axum::async_trait;
use sqlx::{Pool, Postgres};
use crate::domain::entity::user::User;
use crate::domain::repository::user_repository::UserRepository;

pub struct PgUserRepository {
    pool: Pool<Postgres>
}

impl PgUserRepository {
    pub fn new(pool: Pool<Postgres>) -> PgUserRepository {
        PgUserRepository {
            pool
        }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn save(&self, user: &User) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO users(name, email, password) VALUES($1, $2, $3)",
            user.name,
            user.email,
            user.password
        )
        .execute(&self.pool)
        .await
        .expect("User can't create");

        Ok(())
    }

    async fn find_by_id(&self, id: u32) -> Result<Option<User>, Error> {
        todo!()
    }
}