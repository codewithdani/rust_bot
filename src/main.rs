mod db;       // Declare the db module
mod auth;     // Declare the auth module
mod models;   // Declare the models module
mod commands; // Declare the commands module
mod utils;    // Declare the utils module

use teloxide::prelude::*;
use dotenv::dotenv;
use std::env;
use db::get_db_connection; // Import the function from db.rs
use commands::answer;     // Import the answer function from commands.rs

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting bot...");

    // Initialize the database
    if let Err(e) = db::create_tables_if_not_exists() {
        eprintln!("Failed to create tables: {}", e);
        return;
    }

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        match msg.text() {
            Some(text) => {
                commands::handle_command(bot, msg, text).await?;
            }
            None => {
                bot.send_message(msg.chat.id, "I can only understand text messages.").await?;
            }
        }
        Ok(())
    }).await;
}