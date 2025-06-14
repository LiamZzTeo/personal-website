use actix_web::{web, HttpResponse, Responder};
use tera::Tera;

pub async fn home(tera: web::Data<Tera>) -> impl Responder {
    let context = tera::Context::new();
    match tera.render("home.html", &context) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => {
            eprintln!("Template error: {}", e);
            HttpResponse::InternalServerError().body("Template error")
        }
    }
}