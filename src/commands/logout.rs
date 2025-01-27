use asky::Confirm;

use crate::util::{
    db_helpers::{delete_session, get_current_session},
    helpers::{delete_token, get_token},
};

pub async fn logout(pool: &sqlx::Pool<sqlx::Postgres>) {
    let session = get_current_session(pool).await;

    if session.is_none() {
        println!("You are not logged in");
        return;
    }

    let confirmed = Confirm::new("Are you sure you want to logout?")
        .prompt()
        .unwrap();

    match confirmed {
        true => {
            let token = get_token().await.unwrap();
            delete_session(pool, &token).await;
            delete_token().await;

            println!("Logged out");
        }
        false => println!("Logout canceled"),
    }
}
