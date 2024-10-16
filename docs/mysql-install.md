# Ubuntu 22.04でのMySQL 5.6インストールガイド

## インストール手順

1. システムを更新します：
   ```
   sudo apt update
   sudo apt upgrade
   ```

2. 必要な依存関係をインストールします：
   ```
   sudo apt install libaio1 libmecab2 libncurses5
   ```

3. MySQL 5.6のDebianパッケージをダウンロードします：
   ```
   wget https://downloads.mysql.com/archives/get/p/23/file/mysql-server_5.6.51-1debian9_amd64.deb-bundle.tar
   ```

4. ダウンロードしたファイルを解凍します：
   ```
   tar -xvf mysql-server_5.6.51-1debian9_amd64.deb-bundle.tar
   ```

5. 必要なパッケージをインストールします（順番が重要です）：
   ```
   sudo dpkg -i mysql-common_5.6.51-1debian9_amd64.deb
   sudo dpkg -i mysql-community-client_5.6.51-1debian9_amd64.deb
   sudo dpkg -i mysql-client_5.6.51-1debian9_amd64.deb
   sudo dpkg -i mysql-community-server_5.6.51-1debian9_amd64.deb
   ```

6. 不足している依存関係がある場合は、以下のコマンドで解決します：
   ```
   sudo apt --fix-broken install
   ```
   注意: インストール中にrootパスワードの設定を求められます。

7. MySQLサービスが起動していることを確認します：
   ```
   sudo systemctl status mysql
   ```

8. MySQLにログインして動作を確認します：
   ```
   sudo mysql -u root -p
   ```
   プロンプトが表示されたら、設定したrootパスワードを入力します。

## セキュリティ設定

1. MySQL Secure Installationを実行します：
   ```
   sudo mysql_secure_installation
   ```
   このスクリプトに従って、セキュリティ設定を行います。

2. 必要に応じて、` /etc/mysql/mysql.conf.d/mysqld.cnf`ファイルでMySQLの設定を調整します。