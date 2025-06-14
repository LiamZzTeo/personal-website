use actix_web::{web, HttpResponse, Responder};
use tera::Tera;

pub async fn about(tera: web::Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new();
    // 这里可以添加个人介绍数据
    let about_info = "Your personal introduction here";
    context.insert("about_info", &about_info);
    
    let rendered = tera.render("about.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}