use anyhow::Result;
use sqlx::SqlitePool;
use std::process::Command;

pub struct RagService {
    db_pool: SqlitePool,
}

impl RagService {
    pub async fn new(db_url: &str) -> Result<Self> {
        let db_pool = SqlitePool::connect(db_url).await?;
        Ok(Self { db_pool })
    }

    pub async fn query(&self, question: &str) -> Result<String> {
        // 调用Python RAG服务
        let output = Command::new("python")
            .arg("rag_service.py")
            .arg(question)
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}