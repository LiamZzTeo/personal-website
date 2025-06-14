use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PersonalInfo {
    pub id: i64,
    pub name: String,
    pub age: i32,
    pub location: String,
    pub occupation: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Skill {
    pub id: i64,
    pub person_id: i64,
    pub category: String,
    pub name: String,
    pub level: String,
    pub years_of_experience: i32,
    pub projects: String, // JSON string
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Experience {
    pub id: i64,
    pub person_id: i64,
    pub company: String,
    pub position: String,
    pub period: String,
    pub description: String,
    pub achievements: String, // JSON string
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Interest {
    pub id: i64,
    pub person_id: i64,
    pub interest_type: String, // 'book', 'movie', 'tv_show'
    pub title: String,
    pub author_director: String,
    pub year: Option<i32>,
    pub favorite_parts: String, // JSON string
    pub thoughts: String,
}