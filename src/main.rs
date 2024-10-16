use mysql::*;
use mysql::prelude::*;
use tokio;
use futures::future::join_all;
use std::time::{Instant, Duration};
use rand::Rng;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
enum QueryType {
    Select,
    Insert,
    Update,
}

// テーブル作成関数
fn create_table(pool: &Pool) -> Result<()> {
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS test_table (
            id INT AUTO_INCREMENT PRIMARY KEY,
            column1 VARCHAR(255),
            column2 INT
        )"
    )?;
    println!("テーブルが作成されました（存在しない場合）");
    Ok(())
}

// 指定されたクエリタイプに基づいてクエリを実行し、実行時間を返す非同期関数
async fn run_query(pool: &Pool, query_type: QueryType, rng: &mut SmallRng) -> Result<Duration> {
    let start = Instant::now();
    let mut conn = pool.get_conn()?;

    match query_type {
        QueryType::Select => {
            let _: Vec<Row> = conn.query("SELECT * FROM test_table LIMIT 10")?;
        },
        QueryType::Insert => {
            conn.exec_drop(
                "INSERT INTO test_table (column1, column2) VALUES (?, ?)",
                ("テストデータ", rng.gen_range(1..100))
            )?;
        },
        QueryType::Update => {
            let update_value = format!("更新データ {}", rng.gen_range(1..1000));
            conn.exec_drop(
                "UPDATE test_table SET column1 = ? WHERE id = (SELECT id FROM (SELECT id FROM test_table ORDER BY RAND() LIMIT 1) AS temp)",
                (update_value,)
            )?;
        },
    }

    Ok(start.elapsed())
}

// 非同期のメイン処理を行う関数
async fn async_main() -> Result<()> {
    // データベース接続設定
    let url = "mysql://user:password@localhost:3306/database";
    let pool = Pool::new(url)?;

    // テーブルの作成
    create_table(&pool)?;

    let query_types = vec![QueryType::Select, QueryType::Insert, QueryType::Update];

    // テスト設定
    let total_queries = 1000; // 総クエリ数
    let connections = vec![10, 50, 100]; // テストする接続数

    for &conn_count in &connections {
        println!("{}個の接続で{}クエリを実行中", conn_count, total_queries);

        let queries_executed = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();

        // 指定された接続数分の非同期タスクを生成
        for _ in 0..conn_count {
            let pool = pool.clone();
            let query_types = query_types.clone();
            let queries_executed = Arc::clone(&queries_executed);

            let handle = tokio::spawn(async move {
                let mut rng = SmallRng::from_entropy();
                let mut durations = Vec::new();

                while queries_executed.load(Ordering::Relaxed) < total_queries {
                    if queries_executed.fetch_add(1, Ordering::Relaxed) < total_queries {
                        let query_type = query_types[rng.gen_range(0..query_types.len())].clone();
                        match run_query(&pool, query_type, &mut rng).await {
                            Ok(duration) => durations.push(duration),
                            Err(_) => {} // エラーは無視
                        }
                    }
                }
                durations
            });
            handles.push(handle);
        }

        // 全てのタスクの完了を待ち、結果を収集
        let all_durations: Vec<Duration> = join_all(handles)
            .await
            .into_iter()
            .filter_map(|r| r.ok())
            .flatten()
            .collect();

        // 統計情報の計算
        let total_duration: Duration = all_durations.iter().sum();
        let avg_duration = total_duration / all_durations.len() as u32;
        let max_duration = *all_durations.iter().max().unwrap_or(&Duration::from_secs(0));

        // 結果の表示
        println!("平均応答時間: {:?}", avg_duration);
        println!("最大応答時間: {:?}", max_duration);
        println!("合計実行時間: {:?}", total_duration);
        println!("実行されたクエリの総数: {}", all_durations.len());
        println!();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    async_main().await
}