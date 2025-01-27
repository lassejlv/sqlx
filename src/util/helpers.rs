use bcrypt::{hash, verify, DEFAULT_COST};

#[allow(dead_code)]
pub fn hash_password(password: &str) -> String {
    let hashed = hash(password.as_bytes(), DEFAULT_COST).unwrap();
    hashed
}

#[allow(dead_code)]
pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password.as_bytes(), hash).unwrap()
}
