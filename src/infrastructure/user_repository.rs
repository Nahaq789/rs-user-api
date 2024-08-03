use crate::domain::entity::user::User;
use crate::domain::repository::user_repository::UserRepository;
use axum::async_trait;
use sqlx::{Pool, Postgres};
use std::io::Error;

pub struct PgUserRepository {
    pool: Pool<Postgres>,
}

impl PgUserRepository {
    pub fn new(pool: Pool<Postgres>) -> PgUserRepository {
        PgUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn save(&self, user: &User) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO users(name, email, password, salt) VALUES($1, $2, $3, $4)",
            user.name,
            user.email,
            user.password,
            user.salt
        )
        .execute(&self.pool)
        .await
        .expect("User can't create");

        Ok(())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .expect("User can't create");

        Ok(user)
    }

    async fn delete(&self, id: i32) -> Result<(), Error> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .expect("User can't delete");

        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4",
            user.name,
            user.email,
            user.password,
            user.id
        )
        .execute(&self.pool)
        .await
        .expect("User can't update");

        Ok(())
    }
}
