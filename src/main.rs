use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_files as fs;
use tera::Tera;
use std::path::Path;

mod routes;
mod models;

use routes::{home, photography, about, skills};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("Server starting at http://127.0.0.1:8080");

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*.html")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .service(fs::Files::new("/static", concat!(env!("CARGO_MANIFEST_DIR"), "/src/static")).show_files_listing())
            .route("/", web::get().to(home::home))
            .route("/photography", web::get().to(photography::photography))
            .route("/about", web::get().to(about::about))
            .route("/skills", web::get().to(skills::skills))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}