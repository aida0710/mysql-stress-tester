use mysql::prelude::*;
use mysql::*;
use once_cell::sync::Lazy;
use rand::Rng;
use time::{OffsetDateTime, UtcOffset};
use uuid::Uuid;

pub fn create_pool(url: &str) -> Result<Pool, Error> {
    Pool::new(url)
}

// ノードIDを一度だけ生成して再利用
static NODE_ID: Lazy<[u8; 6]> = Lazy::new(|| {
    let mut bytes = [0u8; 6];
    rand::thread_rng().fill(&mut bytes);
    bytes
});

// 起動時に一度だけ実行するクエリ
pub fn single_execute_query<T: Queryable>(conn: &mut T, table_name: &String) -> Result<(), Error> {
    conn.exec_drop(
        format!(
            "CREATE TABLE IF NOT EXISTS `{}` (
                id BIGINT PRIMARY KEY AUTO_INCREMENT,
                utc_db_timestamp TIMESTAMP(2) NOT NULL DEFAULT CURRENT_TIMESTAMP(2),
                utc_app_timestamp TIMESTAMP(2) NOT NULL,
                jst_app_timestamp TIMESTAMP(2) NOT NULL,
                uuidv6 VARCHAR(36) NOT NULL,
                INDEX idx_utc_db_timestamp (utc_db_timestamp),
                INDEX idx_uuidv6 (uuidv6)
            )",
            table_name
        ),
        (),
    )?;

    Ok(())
}

// タイムスタンプをMySQLフォーマットに変換する関数
fn format_mysql_timestamp(dt: OffsetDateTime) -> (String, String) {
    let jst = dt.to_offset(UtcOffset::from_hms(9, 0, 0).unwrap());

    (
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
            dt.year(),
            dt.month() as u8,
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second(),
            dt.microsecond()
        ),
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
            jst.year(),
            jst.month() as u8,
            jst.day(),
            jst.hour(),
            jst.minute(),
            jst.second(),
            jst.microsecond()
        )
    )
}

// 繰り返し実行されるクエリ
pub fn execute_query<T: Queryable>(conn: &mut T, table_name: &str) -> Result<(), Error> {
    let now = OffsetDateTime::now_utc();
    let (utc, jst) = format_mysql_timestamp(now);

    let uuid = Uuid::now_v6(&NODE_ID);

    conn.exec_drop(
        format!(
            "INSERT INTO `{}` (utc_app_timestamp, jst_app_timestamp, uuidv6) VALUES (?, ?, ?)",
            table_name
        ),
        (utc,jst, uuid.to_string()),
    )?;

    Ok(())
}