use postgres::{Client, NoTls, Error};
use std::env;

pub fn get_db_connection() -> Result<Client, Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Client::connect(&db_url, NoTls)
}

// Function to create the tables
fn create_tables_if_not_exists() -> Result<(), Error> {
    let mut client = get_db_connection()?;

    // Create the users table
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            telegram_id BIGINT UNIQUE NOT NULL,
            username TEXT NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
        )",
        &[],
    )?;

    // Create the courses table
    client.execute(
        "CREATE TABLE IF NOT EXISTS courses (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name TEXT NOT NULL,
            description TEXT NOT NULL
        )",
        &[],
    )?;

    // Create the lessons table
    client.execute(
        "CREATE TABLE IF NOT EXISTS lessons (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            course_id UUID NOT NULL REFERENCES courses(id),
            title TEXT NOT NULL,
            content TEXT NOT NULL
        )",
        &[],
    )?;

    // Create the quizzes table
    client.execute(
        "CREATE TABLE IF NOT EXISTS quizzes (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            course_id UUID NOT NULL REFERENCES courses(id),
            name TEXT NOT NULL
        )",
        &[],
    )?;

    // Create the questions table
    client.execute(
        "CREATE TABLE IF NOT EXISTS questions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            quiz_id UUID NOT NULL REFERENCES quizzes(id),
            text TEXT NOT NULL,
            correct_answer TEXT NOT NULL
        )",
        &[],
    )?;

    // Create the user_courses table
    client.execute(
        "CREATE TABLE IF NOT EXISTS user_courses (
            user_id UUID NOT NULL REFERENCES users(id),
            course_id UUID NOT NULL REFERENCES courses(id),
            start_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            completed BOOLEAN NOT NULL DEFAULT FALSE,
            PRIMARY KEY (user_id, course_id)
        )",
        &[],
    )?;

    // Create the user_quiz_attempts table
    client.execute(
        "CREATE TABLE IF NOT EXISTS user_quiz_attempts (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id),
            quiz_id UUID NOT NULL REFERENCES quizzes(id),
            attempt_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            score INTEGER NOT NULL
        )",
        &[],
    )?;

    // Create the user_answers table
    client.execute(
        "CREATE TABLE IF NOT EXISTS user_answers (
            user_quiz_attempt_id UUID NOT NULL REFERENCES user_quiz_attempts(id),
            question_id UUID NOT NULL REFERENCES questions(id),
            answer TEXT NOT NULL,
            PRIMARY KEY (user_quiz_attempt_id, question_id)
        )",
        &[],
    )?;

    Ok(())
}
