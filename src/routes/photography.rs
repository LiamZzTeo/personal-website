use actix_web::{web, HttpResponse, Responder};
use tera::Tera;
use serde::Serialize;

#[derive(Serialize)]
struct Photo {
    filename: &'static str,
    orientation: &'static str,
}

pub async fn photography(tera: web::Data<Tera>) -> impl Responder {
    let photos = vec![
        Photo { filename: "photo2.jpg", orientation: "horizontal" },
        Photo { filename: "photo3.jpg", orientation: "vertical" },
        Photo { filename: "photo4.jpg", orientation: "vertical" },
        Photo { filename: "photo5.jpg", orientation: "vertical" },
        Photo { filename: "photo6.jpg", orientation: "vertical" },
        Photo { filename: "photo7.jpg", orientation: "vertical" },
        Photo { filename: "photo8.jpg", orientation: "vertical" },
    ];

    let mut context = tera::Context::new();
    context.insert("photos", &photos);
    let rendered = tera.render("photography.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}