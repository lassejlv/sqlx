#[allow(dead_code, unused_variables)]
pub async fn me(pool: &sqlx::Pool<sqlx::Postgres>) {
    let user_bin_folder = std::env::var("USER_BIN_FOLDER").expect("USER_BIN_FOLDER must be set");
    println!("User bin folder: {}", user_bin_folder);

    todo!()
}
