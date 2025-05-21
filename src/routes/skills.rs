use actix_web::{web, HttpResponse, Responder};
use tera::Tera;

use serde::Serialize;

#[derive(Serialize)]
struct Skill {
    name: &'static str,
    percent: u8,
    icon: &'static str,
}

pub async fn skills(tera: web::Data<Tera>) -> impl Responder {
    let skills = vec![
        Skill { name: "Photography", percent: 80, icon: "📷" },
        Skill { name: "Python", percent: 90, icon: "🧑🏻‍💻" },
        Skill { name: "C/C++", percent: 70, icon: "💻" },
        Skill { name: "Rust", percent: 70, icon: "🦀📈" },
        Skill { name: "Machine Learning", percent: 90, icon: "🤖" },
        Skill { name: "Financial Analysis", percent: 80, icon: "💰" },
        Skill { name: "Esports", percent: 80, icon: "🎮" },
        Skill { name: "Frontend Development", percent: 85, icon: "🌐" },
        Skill { name: "Deep Learning", percent: 70, icon: "🤖" },
        Skill { name: "Hiking Skills", percent: 60, icon: "⛰️" },
        Skill { name: "Smart Contract", percent: 60, icon: "🔒" },
        Skill { name: "Music Production", percent: 50, icon: "🎵" },
    ];
    let mut context = tera::Context::new();
    context.insert("skills", &skills);
    context.insert("skills_intro", "Learning is a lifelong expedition—curiosity the compass, questions the terrain.");
    let rendered = tera.render("skills.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}