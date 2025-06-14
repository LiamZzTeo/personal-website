use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Content {
    pub id: i64,
    pub title: String,           // 内容标题
    pub content_type: String,    // 内容类型：diary, experience, skill, thought 等
    pub content: String,         // 实际内容
    pub tags: String,           // JSON格式的标签数组
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: String,       // JSON格式的额外元数据
}