use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::env;

#[derive(Deserialize)]
pub struct ChatRequest {
    query: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    response: String,
}

pub async fn chat_endpoint(req: web::Json<ChatRequest>) -> impl Responder {
    // 获取环境变量
    let api_key = env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| "".to_string());
    
    if api_key.is_empty() {
        return HttpResponse::InternalServerError().json(ChatResponse { 
            response: "服务器配置错误：未设置API密钥".to_string()
        });
    }

    // 设置环境变量
    let output = Command::new("python")
        .arg("rag_service.py")
        .arg(&req.query)
        .env("DEEPSEEK_API_KEY", api_key)
        .env("PYTHONIOENCODING", "utf-8")  // 设置Python输出编码
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                // 使用UTF-8解码输出
                match String::from_utf8(output.stdout) {
                    Ok(response) => {
                        if response.trim().is_empty() {
                            HttpResponse::InternalServerError().json(ChatResponse { 
                                response: "服务器返回空响应".to_string()
                            })
                        } else {
                            HttpResponse::Ok()
                                .content_type("application/json; charset=utf-8")
                                .json(ChatResponse { response })
                        }
                    }
                    Err(e) => {
                        HttpResponse::InternalServerError().json(ChatResponse { 
                            response: format!("编码错误: {}", e)
                        })
                    }
                }
            } else {
                let error = String::from_utf8_lossy(&output.stderr).to_string();
                HttpResponse::InternalServerError().json(ChatResponse { 
                    response: format!("服务错误: {}", error)
                })
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ChatResponse { 
                response: format!("系统错误: {}", e)
            })
        }
    }
}