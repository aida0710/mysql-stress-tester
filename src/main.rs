use json::{parse, Error, JsonValue};
use std::fs::File;
use std::io::prelude::*;

mod utils;

fn main() {
    const PREFIX: &str = "[起動処理]";

    println!(
        "{prefix}: 設定ファイルを読み込んでいます。",
        prefix = PREFIX
    );

    let mut setting_file =
        File::open("./config/mysql.json").expect("./config/mysql.jsonが見つかりません。");

    let mut contents = String::new();
    setting_file
        .read_to_string(&mut contents)
        .expect("ファイルの読み込み中に問題がありました。");

    println!("With text:\n{}", contents);

    let parsed = json::parse(&contents).expect("ファイルの文字列パースに失敗しました。");
    if let Some(port) = parsed["port"].as_str() {
        println!("ポート番号: {}", port);
    } else {
        println!("ポート番号が設定ファイルに存在しないか、型が正しくありません。");
    }
}
