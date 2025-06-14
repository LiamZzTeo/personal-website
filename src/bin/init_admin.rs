// src/bin/init_admin.rs
use bcrypt::{hash, DEFAULT_COST};
use dotenv::dotenv;
use std::env;
use std::io::{self, Write};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    print!("请输入管理员用户名: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim();

    // 获取密码
    print!("请输入管理员密码: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    // 获取API Key
    print!("请输入 DEEPSEEK_API_KEY: ");
    io::stdout().flush()?;
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    let api_key = api_key.trim();

    // 生成密码哈希
    let password_hash = hash(password, DEFAULT_COST)?;

    // 生成JWT密钥
    let jwt_secret = base64::encode(rand::random::<[u8; 32]>());

    // 创建.env文件
    let env_content = format!(
        "ADMIN_USERNAME=\"{}\"\nADMIN_PASSWORD_HASH=\"{}\"\nJWT_SECRET=\"{}\"\nDEEPSEEK_API_KEY=\"{}\"\n",
        username, password_hash, jwt_secret, api_key
    );

    fs::write(".env", env_content)?;

    println!("\n管理员账号初始化完成！");
    println!("请确保将.env文件添加到.gitignore中，并妥善保管。");

    Ok(())
}