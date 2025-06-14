use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Photo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub filename: String,
}

#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub level: String,
    pub description: String,
}