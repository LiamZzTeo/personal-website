use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::models::content::Content;

#[derive(Deserialize)]
pub struct CreateContentRequest {
    pub title: String,
    pub content_type: String,
    pub content: String,
    pub tags: Vec<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct ContentResponse {
    pub id: i64,
    pub title: String,
    pub content_type: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub metadata: serde_json::Value,
}

pub async fn create_content(
    db: web::Data<sqlx::SqlitePool>,
    req: web::Json<CreateContentRequest>,
) -> impl Responder {
    let now = Utc::now();
    
    let result = sqlx::query!(
        r#"
        INSERT INTO contents (title, content_type, content, tags, created_at, updated_at, metadata)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        req.title,
        req.content_type,
        req.content,
        serde_json::to_string(&req.tags).unwrap(),
        now,
        now,
        serde_json::to_string(&req.metadata.unwrap_or_default()).unwrap()
    )
    .execute(&**db)
    .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_contents(
    db: web::Data<sqlx::SqlitePool>,
    content_type: Option<web::Query<String>>,
) -> impl Responder {
    let query = match content_type {
        Some(ctype) => {
            sqlx::query_as!(
                Content,
                r#"
                SELECT * FROM contents 
                WHERE content_type = ?
                ORDER BY created_at DESC
                "#,
                ctype.into_inner()
            )
        }
        None => {
            sqlx::query_as!(
                Content,
                r#"
                SELECT * FROM contents 
                ORDER BY created_at DESC
                "#
            )
        }
    };

    match query.fetch_all(&**db).await {
        Ok(contents) => HttpResponse::Ok().json(contents),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}