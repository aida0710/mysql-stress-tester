mod config;
mod error;

use crate::config::Config;
use crate::error::AppError;

fn main() -> Result<(), AppError> {
    const PREFIX: &str = "[起動処理]";

    println!("{}: 設定ファイルを読み込んでいます。", PREFIX);

    let config: Vec<String> = Config::from_file("./config/mysql.json", vec!["port", "username"])?;
    println!("{}: 設定ファイルの内容: {:?}", PREFIX, config);

    Ok(())
}
