# mysql command list

## 基礎
- 起動
  - `sudo systemctl start mysql`
- 終了
  - `sudo systemctl stop mysql`
- 再起動
  - `sudo systemctl restart mysql`
- ステータス確認
  - `sudo systemctl status mysql`
- ログイン
  - `mysql -u root -p`

## データベース操作
- データベース一覧
  - `show databases;`

## rootユーザーリモートアクセス設定
- 現在のrootユーザーの設定を確認
  - `SELECT User, Host FROM mysql.user WHERE User = 'root';`
- 既存の'root'@'%'が存在する場合は削除
  - `DELETE FROM mysql.user WHERE User='root' AND Host='%';`
- リモートからrootユーザーでログインできるように設定
  - `UPDATE mysql.user SET Host='%' WHERE User='root' AND Host='localhost';`
- 設定を反映
  - `FLUSH PRIVILEGES;`
- 再起動
  - `sudo systemctl restart mysql`

## ユーザー操作
- rootユーザーのパスワード設定
  - `ALTER USER 'root'@'localhost' IDENTIFIED BY 'password';`
