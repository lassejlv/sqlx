use dotenv::dotenv;
use std::error::Error;

mod commands;
mod util;

// type users
#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct User {
    id: i32,
    email: String,
    password_hash: String,
    avatar_url: Option<String>,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct Session {
    id: i32,
    user_id: i32,
    token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(&db_url).await?;

    // Apply migrations
    match sqlx::migrate!().run(&pool).await {
        Ok(_) => println!("Applied migrations"),
        Err(e) => println!("Error applying migrations: {}", e),
    }

    let command_name = std::env::args().nth(1);

    match command_name.as_deref() {
        Some("help") => commands::help::help_command().await,
        Some("register") => commands::register::register(&pool).await,
        Some("login") => commands::login::login(&pool).await,
        Some("logout") => commands::logout::logout(&pool).await,
        _ => commands::help::help_command().await,
    }

    Ok(())
}
