use crate::{Session, User};

pub async fn get_session_from_token(pool: &sqlx::Pool<sqlx::Postgres>, token: &str) {
    let session = sqlx::query_as!(Session, "SELECT * FROM sessions WHERE token = $1", token)
        .fetch_optional(pool)
        .await
        .unwrap();

    if session.is_none() {
        println!("Session not found");
        return;
    }

    session.unwrap();
}

pub async fn create_session(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    token: &str,
) -> Session {
    let new_session = sqlx::query_as!(
        Session,
        "INSERT INTO sessions (user_id, token) VALUES ($1, $2) RETURNING *",
        user_id,
        token
    )
    .fetch_one(pool)
    .await
    .unwrap();

    new_session
}

pub async fn delete_session(pool: &sqlx::Pool<sqlx::Postgres>, token: &str) {
    sqlx::query!("DELETE FROM sessions WHERE token = $1", token)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn get_user_by_email(pool: &sqlx::Pool<sqlx::Postgres>, email: &str) -> Option<User> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(pool)
        .await
        .unwrap();

    user
}
