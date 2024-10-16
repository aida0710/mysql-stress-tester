# Mysql Stress Tester
## about
複数のクエリを指定回数実行することで、MySQLの負荷テストを行うツールです。
multi connectionに対応。
## usage

1. このリポジトリをクローンします。
2. Rustの実行環境を用意してください。
3. 依存関係をインストールします。 `cargo build`
4. [.env](.env) ファイルを編集し、MySQLの接続情報と繰り返し回数、connection数を設定してください。
5. [database.rs](src/database.rs)を編集して実行するクエリを設定してください。
6. 実行 `cargo run`