use bcrypt::{BcryptError, verify};

pub fn verify_password(hash: &str, plain: &str) -> Result<bool, BcryptError> {
    verify(plain, hash)
}