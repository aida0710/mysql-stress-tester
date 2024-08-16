use json::{parse, JsonValue};
use std::fs::File;
use std::io::prelude::*;

mod utils;

fn main() {
    const PREFIX: &str = "[起動処理]";

    println!(
        "{prefix}: 設定ファイルを読み込んでいます。",
        prefix = PREFIX
    );

    let parsed = json_parser("./config/mysql.json").expect("JSONのパースに失敗しました。");

    if let Some(port) = parsed["port"].as_str() {
        println!("ポート番号: {}", port);
    } else {
        println!("ポート番号が設定ファイルに存在しないか、型が正しくありません。");
    }
}

fn json_parser(path: &str) -> Result<JsonValue, Err> {
    let mut setting_file = File::open(path).expect("ファイルが見つかりません。");
    let mut contents = String::new();
    setting_file
        .read_to_string(&mut contents)
        .expect("ファイルの読み込み中に問題がありました。");

    println!("With text:\n{}", contents);

    let parsed = parse(&contents).expect("JSONのパースに失敗しました。");

    Ok(parsed)
}
