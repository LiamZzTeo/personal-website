use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use crate::utils::auth::{hash_password, verify_password};
use tera::{Tera, Context};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContentRequest {
    title: String,
    content_type: String,
    content: String,
    tags: String,
    metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    id: i64,
    title: String,
    content_type: String,
    content: String,
    tags: String,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
    metadata: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/login", web::post().to(login))
        .route("/check-auth", web::get().to(check_auth))
        .route("/logout", web::post().to(logout))
        .route("/content", web::get().to(get_content))
        .route("/content", web::post().to(add_content))
        .route("/content/{id}", web::get().to(get_content_by_id))  // 添加这行
        .route("/content/{id}", web::put().to(update_content))     // 添加这行
        .route("/content/{id}", web::delete().to(delete_content));
}

use std::fs::OpenOptions;
use std::io::Write;

fn log_to_file(msg: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log")
        .unwrap();
    writeln!(file, "{}", msg).unwrap();
}

// 页面处理函数
pub async fn login_page(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let rendered = tmpl.render("admin/login.html", &ctx).unwrap();
    println!("login_page 被访问");
    
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn dashboard_page(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = Context::new();
    let rendered = tmpl.render("admin/dashboard.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

// API处理函数
pub async fn login(
    req: web::Json<LoginRequest>,
) -> impl Responder {
    println!("login 函数被调用");
    log_to_file("login 函数被调用");

    // 直接从文件读取哈希值
    let admin_password_hash = match std::fs::read_to_string(".env") {
        Ok(content) => {
            let hash_line = content.lines()
                .find(|line| line.starts_with("ADMIN_PASSWORD_HASH="))
                .unwrap_or("");
            
            if hash_line.is_empty() {
                println!("未找到 ADMIN_PASSWORD_HASH");
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "ADMIN_PASSWORD_HASH not found in .env"
                }));
            }

            // 提取哈希值，保留所有字符
            let hash = hash_line.split('=').nth(1).unwrap_or("").trim_matches('"');
            println!("原始哈希值: {}", hash);
            println!("哈希长度: {}", hash.len());
            
            // 验证是否是完整的 bcrypt 哈希
            if !hash.starts_with("$2b$") {
                println!("哈希值格式错误，缺少 bcrypt 前缀");
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Invalid bcrypt hash format"
                }));
            }
            
            hash.to_string()
        },
        Err(e) => {
            println!("读取 .env 文件失败: {:?}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to read .env file"
            }));
        }
    };

    // 验证哈希格式
    if admin_password_hash.len() != 60 {
        println!("密码哈希长度不正确: {}", admin_password_hash.len());
        println!("哈希值: {}", admin_password_hash);
        log_to_file(&format!("密码哈希长度不正确: {}", admin_password_hash.len()));
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Server configuration error: Invalid password hash format"
        }));
    }


    // 密码校验
    match verify_password(&req.password, &admin_password_hash) {
        Ok(true) => {
            println!("密码校验通过");
            log_to_file("密码校验通过");
            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .expect("valid timestamp")
                .timestamp() as usize;

            let claims = Claims {
                sub: req.username.clone(),
                exp: expiration,
            };

            let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
            
            match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(jwt_secret.as_bytes())
            ) {
                Ok(token) => {
                    println!("token 生成成功");
                    log_to_file("token 生成成功");
                    HttpResponse::Ok().json(serde_json::json!({
                        "token": token
                    }))
                },
                Err(e) => {
                    println!("token 生成失败: {:?}", e);
                    log_to_file(&format!("token 生成失败: {:?}", e));
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to generate token"
                    }))
                }
            }
        }
        Ok(false) => {
            println!("密码校验失败");
            // 添加更多调试信息
            println!("密码长度: {}", req.password.len());
            println!("哈希长度: {}", admin_password_hash.len());
            // 打印每个字符的 ASCII 值
            println!("密码字符 ASCII 值:");
            for (i, c) in req.password.chars().enumerate() {
                println!("位置 {}: '{}' (ASCII: {})", i, c, c as u8);
            }
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid credentials"
            }))
        }
        Err(e) => {
            println!("密码校验出错: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Password verification error"
            }))
        }
    }
}

async fn check_auth(
    req: HttpRequest,
) -> impl Responder {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(token) = auth_header.to_str() {
            println!("收到的 token: {}", token);
            let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
            println!("用于解码的 JWT_SECRET: {:?}", jwt_secret);
            if token.starts_with("Bearer ") {
                let token = token.replace("Bearer ", "");
                let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(jwt_secret.as_bytes()),
                    &Validation::default()
                ) {
                    Ok(_) => return HttpResponse::Ok().json(serde_json::json!({
                        "last_login": Utc::now().to_rfc3339()
                    })),
                    Err(_) => return HttpResponse::Unauthorized().finish(),
                }
            }
        }
    }
    HttpResponse::Unauthorized().finish()
}

async fn logout() -> impl Responder {
    HttpResponse::Ok().finish()
}

// 获取内容列表
pub async fn get_content(
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    println!("开始获取内容列表");  // 添加日志
    
    match sqlx::query(
        "SELECT id, title, content_type, content, tags, created_at, updated_at, metadata 
         FROM contents 
         ORDER BY created_at DESC"
    )
    .fetch_all(&**pool)
    .await
    {
        Ok(rows) => {
            println!("成功获取到 {} 条记录", rows.len());  // 添加日志
            let contents: Vec<Content> = rows.iter().map(|row| Content {
                id: row.get("id"),
                title: row.get("title"),
                content_type: row.get("content_type"),
                content: row.get("content"),
                tags: row.get("tags"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                metadata: row.get("metadata"),
            }).collect();
            HttpResponse::Ok().json(contents)
        },
        Err(e) => {
            println!("获取内容失败: {:?}", e);  // 添加日志
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to fetch content: {}", e)
            }))
        }
    }
}

// 添加新内容
async fn add_content(
    req: web::Json<ContentRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    match sqlx::query(
        "INSERT INTO contents (title, content_type, content, tags, metadata) 
         VALUES (?, ?, ?, ?, ?) 
         RETURNING id"
    )
    .bind(&req.title)
    .bind(&req.content_type)
    .bind(&req.content)
    .bind(&req.tags)
    .bind(req.metadata.as_deref().unwrap_or("{}"))
    .fetch_one(&**pool)
    .await
    {
        Ok(row) => {
            // 调用 Python 脚本让 RAG 重新加载
            let _ = Command::new("python")
                .arg("rag_service.py")
                .arg("--reload")
                .current_dir(".") // 修改为当前目录
                .output();
        
            HttpResponse::Ok().json(serde_json::json!({
                "id": row.get::<i64, _>("id")
            }))
        },
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to add content"
        }))
    }
}

// 删除内容
async fn delete_content(
    id: web::Path<i64>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    match sqlx::query(
        "DELETE FROM contents WHERE id = ?"
    )
    .bind(id.into_inner())
    .execute(&**pool)
    .await
    {
        Ok(_) => {
            // 调用 Python 脚本让 RAG 重新加载
            let _ = Command::new("python")
                .arg("rag_service.py")
                .arg("--reload")
                .current_dir(".")
                .output();
            
            HttpResponse::Ok().finish()
        },
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to delete content"
        }))
    }
}

// 添加获取单个内容的处理函数
async fn get_content_by_id(
    id: web::Path<i64>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    println!("获取内容 ID: {}", id);  // 添加日志

    match sqlx::query(
        "SELECT id, title, content_type, content, tags, created_at, updated_at, metadata 
         FROM contents 
         WHERE id = ?"
    )
    .bind(id.into_inner())
    .fetch_one(&**pool)
    .await
    {
        Ok(row) => {
            let content = Content {
                id: row.get("id"),
                title: row.get("title"),
                content_type: row.get("content_type"),
                content: row.get("content"),
                tags: row.get("tags"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                metadata: row.get("metadata"),
            };
            HttpResponse::Ok().json(content)
        },
        Err(e) => {
            println!("获取内容失败: {:?}", e);  // 添加日志
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to fetch content: {}", e)
            }))
        }
    }
}

async fn update_content(
    id: web::Path<i64>,
    req: web::Json<ContentRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    println!("更新内容 ID: {}", id);

    match sqlx::query(
        "UPDATE contents 
         SET title = ?, content_type = ?, content = ?, tags = ?, metadata = ?, updated_at = CURRENT_TIMESTAMP 
         WHERE id = ?"
    )
    .bind(&req.title)
    .bind(&req.content_type)
    .bind(&req.content)
    .bind(&req.tags)
    .bind(req.metadata.as_deref().unwrap_or("{}"))
    .bind(id.into_inner())
    .execute(&**pool)
    .await
    {
        Ok(_) => {
            // 调用 Python 脚本让 RAG 重新加载
            let _ = Command::new("python")
                .arg("rag_service.py")
                .arg("--reload")
                .current_dir(".") // 修改为当前目录
                .output();
        
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            println!("更新内容失败: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to update content: {}", e)
            }))
        }
    }
}