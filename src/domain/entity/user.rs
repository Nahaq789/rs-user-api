#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String
}

impl User {
    fn new(id: i32, name: String, email: String, password: String) -> User {
        User {
            id, name, email, password
        }
    }
}