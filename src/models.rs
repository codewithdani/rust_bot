use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use uuid::Uuid;

// Define a User struct (adjust to match your Supabase table)
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<uuid::Uuid>,  // Option because the DB generates it
    telegram_id: i64,
    username: String,
    created_at: DateTime<Utc>,
}

// Define a Course struct
#[derive(Debug, Serialize, Deserialize)]
struct Course {
    id: uuid::Uuid,
    name: String,
    description: String,
}

// Define a Quiz struct
#[derive(Debug, Serialize, Deserialize)]
struct Quiz {
    id: uuid::Uuid,
    course_id: uuid::Uuid,
    name: String,
}

// Define a Question struct
#[derive(Debug, Serialize, Deserialize)]
struct Question {
    id: uuid::Uuid,
    quiz_id: uuid::Uuid,
    text: String,
    correct_answer: String, // or use index
}

// UserQuizAttempt struct
#[derive(Debug, Serialize, Deserialize)]
struct UserQuizAttempt {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    quiz_id: uuid::Uuid,
    attempt_date: DateTime<Utc>,
    score: i32,
}

//User Answer struct
#[derive(Debug, Serialize, Deserialize)]
struct UserAnswer {
    user_quiz_attempt_id: uuid::Uuid,
    question_id: uuid::Uuid,
    answer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCourse {
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub content: String,
}
