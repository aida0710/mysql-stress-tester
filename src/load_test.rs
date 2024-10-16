use crate::config::Config;
use crate::database::{create_pool, execute_query, single_execute_query};
use crate::error::{Result, LoadTestError};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Instant, Duration};
use mysql::Pool;
use tokio::time::interval;

async fn execute_single_query(pool: &Pool) -> Result<()> {
    println!("事前クエリを実行中...");
    let start_time = Instant::now();

    let mut conn = pool.get_conn().map_err(LoadTestError::from)?;
    single_execute_query(&mut conn).map_err(LoadTestError::from)?;

    let duration = start_time.elapsed();
    println!("事前クエリ実行完了。実行時間: {:?}", duration);

    Ok(())
}

pub async fn run_load_test(config: Config) -> Result<()> {
    let pool = create_pool(&config.database_url)?;

    // 事前に1回クエリを実行
    execute_single_query(&pool).await?;

    println!("{}個の接続で{}クエリを実行中", config.connections, config.total_queries);

    let queries_executed = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    let start_time = Instant::now();
    let queries_executed_clone = Arc::clone(&queries_executed);
    let total_queries = config.total_queries;

    // 毎秒のクエリ処理数を出力するタスク
    let monitoring_handle = tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));
        let mut last_count = 0;
        loop {
            interval.tick().await;
            let current_count = queries_executed_clone.load(Ordering::Relaxed);
            let queries_per_second = current_count - last_count;
            let elapsed = start_time.elapsed();
            println!(
                "経過時間: {:.3}秒 ({} ms), 毎秒のクエリ処理数: {}, 総クエリ数: {}",
                elapsed.as_secs_f64(),
                elapsed.as_millis(),
                queries_per_second,
                current_count
            );
            last_count = current_count;

            if current_count >= total_queries {
                break;
            }
        }
    });

    for _ in 0..config.connections {
        let pool = pool.clone();
        let queries_executed = Arc::clone(&queries_executed);
        let total_queries = config.total_queries;

        let handle: tokio::task::JoinHandle<Result<()>> = tokio::spawn(async move {
            let mut conn = pool.get_conn().map_err(LoadTestError::from)?;

            while queries_executed.load(Ordering::Relaxed) < total_queries {
                if queries_executed.fetch_add(1, Ordering::Relaxed) < total_queries {
                    execute_query(&mut conn).map_err(LoadTestError::from)?;
                }
            }
            Ok(())
        });
        handles.push(handle);
    }

    // タスクの結果を処理
    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("タスクの実行中にエラーが発生しました: {}", e);
        }
    }

    // モニタリングタスクを終了
    monitoring_handle.abort();

    let total_duration = start_time.elapsed();
    let total_queries_executed = queries_executed.load(Ordering::Relaxed);
    let avg_queries_per_second = total_queries_executed as f64 / total_duration.as_secs_f64();

    println!("テスト完了");
    println!("合計実行時間: {:?}", total_duration);
    println!("実行されたクエリの総数: {}", total_queries_executed);
    println!("平均クエリ/秒: {:.2}", avg_queries_per_second);

    Ok(())
}