use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub salt: String,
}

// impl User {
//     fn new(id: i32, name: String, email: String, password: String) -> User {
//         User {
//             id, name, email, password
//         }
//     }
// }
