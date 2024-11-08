use dotenv::dotenv;
use std::env;
use crate::error::Result;

pub struct Config {
    pub database_url: String,
    pub total_batches: usize,  // 総バッチ数
    pub connections: usize,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        let database_user = env::var("DATABASE_USER")?;
        let database_password = env::var("DATABASE_PASSWORD")?;
        let database_host = env::var("DATABASE_HOST")?;
        let database_port = env::var("DATABASE_PORT")?;
        let database_name = env::var("DATABASE_NAME")?;

        let database_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            database_user, database_password, database_host, database_port, database_name
        );

        // バッチの総数 (デフォルト: 1000 = 100万レコード)
        let total_batches: usize = env::var("TOTAL_BATCHES")
            .unwrap_or_else(|_| "1000".to_string())
            .parse()?;

        // 接続数 (デフォルト: 10)
        let connections: usize = env::var("CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()?;

        Ok(Config {
            database_url,
            total_batches,
            connections,
        })
    }
}