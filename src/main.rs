use actix_web::{web, App, HttpServer};
use actix_files as actix_fs;
use tera::Tera;
use sqlx::sqlite::SqlitePool;
use std::env;
use dotenv::dotenv;
use std::fs::read_to_string;
use std::fs;
mod routes;
mod models;
mod services;
mod utils;

use routes::{home, photography, about, skills, chat, admin};
use services::rag_service::RagService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化环境变量
    dotenv().ok();
    let admin_password_hash = std::env::var("ADMIN_PASSWORD_HASH").unwrap();
    println!("哈希长度: {}", admin_password_hash.len());
    println!("哈希内容: {:?}", admin_password_hash);
    let env_content = fs::read_to_string(".env").unwrap();
    println!("哈希值的每个字符:");
    for (i, c) in admin_password_hash.chars().enumerate() {
        println!("位置 {}: '{}' (ASCII: {})", i, c, c as u8);
    }
    println!(".env 文件内容：\n{}", env_content);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    

    // 设置数据库连接
    let database_url = "sqlite:personal_info.db";
    let pool = SqlitePool::connect(database_url)
        .await
        .expect("Failed to create pool");

    // 初始化数据库
    let init_sql = read_to_string("scripts/init_db.sql")
        .expect("Failed to read init_db.sql");
    
    sqlx::query(&init_sql)
        .execute(&pool)
        .await
        .expect("Failed to initialize database");

    println!("Server starting at http://127.0.0.1:8080");

    // 初始化RAG服务
    let rag_service = web::Data::new(
        RagService::new(database_url)
            .await
            .expect("Failed to initialize RAG service")
    );

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*.html")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(pool.clone()))
            .app_data(rag_service.clone())
            // API路由
            .service(web::resource("/api/chat").route(web::post().to(chat::chat_endpoint)))
            .service(web::scope("/api/admin").configure(admin::config))
            // 静态文件
            .service(actix_fs::Files::new("/static", concat!(env!("CARGO_MANIFEST_DIR"), "/src/static")).show_files_listing())
            // 页面路由
            .route("/", web::get().to(home::home))
            .route("/photography", web::get().to(photography::photography))
            .route("/about", web::get().to(about::about))
            .route("/skills", web::get().to(skills::skills))
            .route("/admin/login", web::get().to(admin::login_page))
            .route("/admin/dashboard", web::get().to(admin::dashboard_page))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}