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
        Photo { filename: "photo2.jpeg", orientation: "horizontal" },
        Photo { filename: "photo3.jpeg", orientation: "vertical" },
        Photo { filename: "photo4.jpeg", orientation: "vertical" },
        Photo { filename: "photo5.jpeg", orientation: "vertical" },
        Photo { filename: "photo6.jpeg", orientation: "vertical" },
        Photo { filename: "photo7.jpeg", orientation: "vertical" },
        Photo { filename: "photo8.jpeg", orientation: "vertical" },
    ];

    let mut context = tera::Context::new();
    context.insert("photos", &photos);
    let rendered = tera.render("photography.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}