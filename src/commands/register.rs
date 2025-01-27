use crate::util::{db_helpers::get_current_session, helpers::hash_password};
use asky::{Password, Text};

use crate::User;

pub async fn register(pool: &sqlx::Pool<sqlx::Postgres>) {
    let session = get_current_session(pool).await;

    if session.is_some() {
        println!("You are already logged in");
        return;
    }

    let email = Text::new("Whats your email?").prompt().unwrap();
    let password = Password::new("Whats your password?").prompt().unwrap();

    let hashed_password = hash_password(&password);

    let new_user = sqlx::query_as!(
        User,
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING *",
        email,
        hashed_password
    )
    .fetch_one(pool)
    .await
    .unwrap();

    println!("Registered user: {:#?}", new_user.email);
    println!("You can now use the login command");
}
