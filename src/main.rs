mod config;
mod error;

use crate::config::Config;
use crate::error::AppError;
use mysql::{prelude::*, Pool};

fn main() -> Result<(), AppError> {
    const PREFIX: &str = "[起動処理]";
    println!("{}: 設定ファイルを読み込んでいます", PREFIX);
    let config: Vec<String> = Config::from_file("./config/mysql.json", vec![
        "address",
        "port",
        "username",
        "password",
    ])?;
    println!("{}: 設定ファイルの内容: {:?}", PREFIX, config);
    println!("{}: 設定ファイルの読み込みが完了しました", PREFIX);

    let url = format!(
        "mysql://{username}:{password}@{address}:{port}",
        username = config[2],
        password = config[3],
        address = config[0],
        port = config[1]
    );
    let pool = Pool::new(url.as_str())?;
    let mut connect = pool.get_conn().expect("接続に失敗しました");

    loop {
        let mut input = String::new();
        println!("SQLクエリを入力してください:");
        std::io::stdin().read_line(&mut input).expect("標準入力に失敗しました");
        if input.trim().is_empty() {
            println!("空文字列は無効です");
            continue;
        }

        if input.trim().to_lowercase() == "exit" {
            println!("プログラムを終了します");
            break;
        }

        match connect.query_iter(input.trim()) {
            Ok(result) => {
                for row in result {
                    match row {
                        Ok(row) => println!("{:?}", row),
                        Err(e) => println!("行の取得中にエラーが発生しました: {}", e),
                    }
                }
            }
            Err(e) => println!("クエリの実行に失敗しました: {}", e),
        }
    }
    Ok(())
}