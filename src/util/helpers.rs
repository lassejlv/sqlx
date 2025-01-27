use bcrypt::{hash, verify, DEFAULT_COST};

use super::config_dir::ensure_app_dirs;

#[allow(dead_code)]
pub fn hash_password(password: &str) -> String {
    let hashed = hash(password.as_bytes(), DEFAULT_COST).unwrap();
    hashed
}

#[allow(dead_code)]
pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password.as_bytes(), hash).unwrap()
}

pub async fn check_token_file() -> bool {
    let path = format!("{}/token.txt", ensure_app_dirs().await.unwrap());

    if !tokio::fs::read_to_string(path).await.is_ok() {
        return false;
    } else {
        return true;
    }
}

pub async fn set_token(token: &str) -> std::io::Result<()> {
    let path = format!("{}/token.txt", ensure_app_dirs().await.unwrap());
    tokio::fs::write(path, token).await
}

pub async fn delete_token() -> std::io::Result<()> {
    let path = format!("{}/token.txt", ensure_app_dirs().await.unwrap());
    tokio::fs::remove_file(path).await
}

pub async fn get_token() -> std::io::Result<String> {
    let path = format!("{}/token.txt", ensure_app_dirs().await.unwrap());
    tokio::fs::read_to_string(path).await
}
