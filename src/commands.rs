use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use crate::auth::register_user; // Import the register_user function
use crate::db::get_db_connection;
use uuid::Uuid;

// Bot commands
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Register a new user")]
    Register,
    #[command(description = "List available courses")]
    Courses,
    #[command(description = "Show my progress")]
    Progress,
    #[command(description = "Begin a quiz")]
    Quiz { course_id: String }, // /quiz <course_id>
}

pub async fn handle_command(bot: Bot, msg: Message, text: &str) -> Result<(), teloxide::RequestError> {
    match Command::parse(text, None) {
        Ok(command) => {
            match command {
                Command::Start => {
                    bot.send_message(msg.chat.id, "Welcome to the course bot!").await?;
                }
                Command::Register => {
                    register_user(&bot, &msg).await?;
                }
                Command::Courses => {
                    list_courses(&bot, &msg).await?;
                }
                Command::Progress => {
                    bot.send_message(msg.chat.id, "Progress feature coming soon!").await?;
                }
                Command::Quiz { course_id } => {
                    start_quiz(&bot, &msg, course_id).await?;
                }
            }
        }
        Err(_) => {
            bot.send_message(msg.chat.id, "Unknown command. Type /help for a list of commands.").await?;
        }
    }
    Ok(())
}

async fn list_courses(bot: &Bot, msg: &Message) -> Result<(), teloxide::RequestError> {
    let mut client = get_db_connection().expect("Failed to connect to database");

    let rows = client.query("SELECT id, name, description FROM courses", &[]).expect("Failed to fetch courses");

    if rows.is_empty() {
        bot.send_message(msg.chat.id, "No courses available yet.").await?;
        return Ok(());
    }

    for row in rows {
        let course_id: Uuid = row.get("id");
        let course_name: String = row.get("name");
        let course_description: String = row.get("description");

        bot.send_message(msg.chat.id, format!("{}: {}", course_name, course_description)).await?;
    }

    Ok(())
}

async fn start_quiz(bot: &Bot, msg: &Message, course_id: String) -> Result<(), teloxide::RequestError> {
    bot.send_message(msg.chat.id, format!("Starting quiz for course ID: {}", course_id)).await?;
    Ok(())
}