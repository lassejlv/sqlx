use crate::util::config_dir::ensure_app_dirs;

#[allow(unused_variables)]
pub async fn login(pool: &sqlx::Pool<sqlx::Postgres>) {
    let config_dir = ensure_app_dirs().await.unwrap();
    println!("Config dir: {}", config_dir);
}
