use postgres::{Client, Error};
use teloxide::prelude::*;
use crate::db::get_db_connection; // Use crate:: to refer to the root of your project
use crate::models::User;
use chrono::Utc;

pub async fn register_user(bot: &Bot, msg: &Message) -> Result<(), teloxide::RequestError> {
    let telegram_id: i64 = msg.from().map(|user| user.id.0).expect("User ID not found");
    let username: String = msg.from()
    .and_then(|user| user.username.clone())
    .unwrap_or_else(|| "unknown".to_string());

    let mut client = get_db_connection().expect("Failed to connect to database");

    // Check if the user already exists
    let user_exists = client.query(
        "SELECT id FROM users WHERE telegram_id = $1",
        &[&telegram_id],
    ).map(|rows| !rows.is_empty()).unwrap_or(false);

    if user_exists {
        bot.send_message(msg.chat.id, "You are already registered!").await?;
        return Ok(());
    }

    // Insert the new user into the database
    let new_user = User {
        id: None,
        telegram_id,
        username,
        created_at: Utc::now(),
    };

    let result = client.execute(
        "INSERT INTO users (telegram_id, username, created_at) VALUES ($1, $2, $3)",
        &[&new_user.telegram_id, &new_user.username, &new_user.created_at],
    );

    match result {
        Ok(_) => {
            bot.send_message(msg.chat.id, "Successfully registered!").await?;
        }
        Err(e) => {
            eprintln!("Failed to register user: {}", e);
            bot.send_message(msg.chat.id, "Failed to register. Please try again later.").await?;
        }
    }

    Ok(())
}