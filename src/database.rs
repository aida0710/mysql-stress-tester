use mysql::prelude::*;
use mysql::*;
use once_cell::sync::Lazy;
use rand::Rng;
use time::{OffsetDateTime, UtcOffset};
use uuid::Uuid;

pub const BATCH_SIZE: usize = 1000;

// ノードIDを一度だけ生成して再利用
static NODE_ID: Lazy<[u8; 6]> = Lazy::new(|| {
    let mut bytes = [0u8; 6];
    rand::thread_rng().fill(&mut bytes);
    bytes
});

pub fn create_pool(url: &str) -> Result<Pool, Error> {
    let opts = Opts::from_url(url)?;
    Pool::new(opts)
}

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
                request_type VARCHAR(10) NOT NULL,
                payload TEXT,
                processing_time_ms INT NOT NULL,
                status_code SMALLINT NOT NULL,
                client_ip VARCHAR(45) NOT NULL,
                user_agent VARCHAR(255) NOT NULL,
                INDEX idx_utc_db_timestamp (utc_db_timestamp),
                INDEX idx_uuidv6 (uuidv6),
                INDEX idx_request_type (request_type),
                INDEX idx_status_code (status_code)
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

// バッチデータの生成
fn generate_batch_data(size: usize) -> Vec<(String, String, String, String, String, i32, i16, String, String)> {
    let mut rng = rand::thread_rng();
    let request_types = ["GET", "POST", "PUT", "DELETE"];
    let user_agents = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 14_7_1 like Mac OS X)",
        "Mozilla/5.0 (Linux; Android 11; Pixel 5)",
    ];

    (0..size)
        .map(|_| {
            let now = OffsetDateTime::now_utc();
            let (utc, jst) = format_mysql_timestamp(now);
            let uuid = Uuid::now_v6(&NODE_ID);
            let request_type = request_types[rng.gen_range(0..request_types.len())];

            let payload = format!(
                "{{\"user_id\": {}, \"action\": \"test\", \"data\": \"sample{}\"}}",
                rng.gen_range(1..10000),
                rng.gen_range(1..100)
            );

            let processing_time = rng.gen_range(1..1000);
            let status_code = if rng.gen_ratio(9, 10) { 200 } else { 500 };
            let client_ip = format!(
                "{}.{}.{}.{}",
                rng.gen_range(1..255),
                rng.gen_range(1..255),
                rng.gen_range(1..255),
                rng.gen_range(1..255)
            );
            let user_agent = user_agents[rng.gen_range(0..user_agents.len())];

            (
                utc,
                jst,
                uuid.to_string(),
                request_type.to_string(),
                payload,
                processing_time,
                status_code,
                client_ip,
                user_agent.to_string(),
            )
        })
        .collect()
}

// 繰り返し実行されるクエリ（バルクインサート版）
pub fn execute_query<T: Queryable>(conn: &mut T, table_name: &str) -> Result<(), Error> {
    let batch_data = generate_batch_data(BATCH_SIZE);

    conn.exec_batch(
        format!(
            "INSERT INTO `{}` (
                utc_app_timestamp,
                jst_app_timestamp,
                uuidv6,
                request_type,
                payload,
                processing_time_ms,
                status_code,
                client_ip,
                user_agent
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            table_name
        ),
        batch_data,
    )?;

    Ok(())
}