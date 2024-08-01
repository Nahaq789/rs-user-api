use password_hash::PasswordHasher;
use pbkdf2::{
    password_hash::{rand_core::OsRng, SaltString},
    Params, Pbkdf2,
};

pub trait CryptoService {
    fn password_hash(&self, password: &[u8]) -> String;
}

pub struct CryptoServiceImpl {}

impl CryptoServiceImpl {
    pub fn new() -> CryptoServiceImpl {
        CryptoServiceImpl {}
    }
}

impl CryptoService for CryptoServiceImpl {
    fn password_hash(&self, password: &[u8]) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let param: Params = Params {
            rounds: (10000),
            output_length: (32),
        };
        match Pbkdf2.hash_password_customized(password, None, None, param, &salt) {
            Ok(hash_password) => hash_password.to_string(),
            Err(_) => "Error: Failed to hash password".to_string(),
        }
    }
}
