mod config;
mod error;

use mysql::{Pool, prelude::*};
use crate::config::Config;
use crate::error::AppError;

fn main() -> Result<(), AppError> {
    const PREFIX: &str = "[起動処理]";
    println!("{}: 設定ファイルを読み込んでいます", PREFIX);
    let config: Vec<String> = Config::from_file("./config/mysql.json", vec!["port", "username"])?;
    println!("{}: 設定ファイルの内容: {:?}", PREFIX, config);
    println!("{}: 設定ファイルの読み込みが完了しました", PREFIX);

    let url = "mysql://root:password@localhost:3306/db_name";
    let pool = Pool::new(url)?;
    let mut connect = pool.get_conn()?;

    connect.query_drop( r"create database test")?;

    Ok(())
}