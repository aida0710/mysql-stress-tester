mod error;
mod config;
mod database;
mod load_test;

use crate::error::Result;
use crate::config::Config;
use crate::load_test::run_load_test;

#[tokio::main]
async fn main() {
    if let Err(e) = async_main().await {
        eprintln!("エラーが発生しました: {}", e);
        std::process::exit(1);
    }
}

async fn async_main() -> Result<()> {
    let config = Config::from_env()?;
    run_load_test(config).await
}