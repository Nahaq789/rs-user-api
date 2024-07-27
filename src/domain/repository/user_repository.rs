use std::io::Error;

pub trait UserRepository: Send + Sync {
    async fn save(user: &User) -> Result<(), Error>;
    async fn find_by_id(id: u32) -> Result<Option<User>, Error>;
}