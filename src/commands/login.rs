use asky::{Password, Text};
use uuid::Uuid;

use crate::{
    util::{
        config_dir::ensure_app_dirs,
        db_helpers::{create_session, get_current_session},
        helpers::{set_token, verify_password},
    },
    User,
};

#[allow(unused_variables)]
pub async fn login(pool: &sqlx::Pool<sqlx::Postgres>) {
    let config_dir = ensure_app_dirs().await.unwrap();

    let session = get_current_session(pool).await;

    if session.is_some() {
        println!("You are already logged in");
        return;
    }

    let email = Text::new("Whats your email?").prompt().unwrap();
    let password = Password::new("Whats your password?").prompt().unwrap();

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_one(pool)
        .await
        .unwrap();

    match verify_password(&password, &user.password_hash) {
        true => {
            let token = Uuid::now_v7().to_string();
            let session = create_session(pool, user.id, &token).await;
            set_token(&token).await.unwrap();

            println!("Login successful");
            println!("Token: {}", session.token);
            println!("User ID: {}", session.user_id);
        }
        false => {
            println!("Login failed");
        }
    }
}
