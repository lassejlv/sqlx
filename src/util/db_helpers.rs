use crate::Session;

use super::helpers::{check_token_file, get_token};

pub async fn get_session_from_token(
    pool: &sqlx::Pool<sqlx::Postgres>,
    token: &str,
) -> Option<Session> {
    let session = sqlx::query_as!(Session, "SELECT * FROM sessions WHERE token = $1", token)
        .fetch_optional(pool)
        .await
        .unwrap();

    if session.is_none() {
        println!("Session not found");
        return None;
    }

    Some(session.unwrap())
}

pub async fn create_session(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    token: &str,
) -> Session {
    sqlx::query_as!(
        Session,
        "INSERT INTO sessions (user_id, token) VALUES ($1, $2) RETURNING *",
        user_id,
        token
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

pub async fn delete_session(pool: &sqlx::Pool<sqlx::Postgres>, token: &str) {
    sqlx::query!("DELETE FROM sessions WHERE token = $1", token)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn get_current_session(pool: &sqlx::Pool<sqlx::Postgres>) -> Option<Session> {
    let exist = check_token_file().await;

    if !exist {
        None
    } else {
        let token = get_token().await.unwrap();
        

        get_session_from_token(pool, &token).await
    }
}
