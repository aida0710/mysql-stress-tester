use crate::config::Config;
use crate::database::{create_pool, execute_query, single_execute_query, BATCH_SIZE};
use crate::error::{LoadTestError, Result};
use mysql::Pool;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use time::{Duration as Time_Duration, OffsetDateTime};
use tokio::time::interval;
use tokio::sync::mpsc;
use std::io::{self, Write};

// モニタリング用の構造体
#[derive(Debug)]
struct MetricsUpdate {
    batches_executed: usize,
    total_records: usize,
    elapsed_time: Duration,
}

async fn execute_single_query(pool: &Pool) -> Result<String> {
    print!("事前クエリを実行中...");
    io::stdout().flush().unwrap();
    let start_time = Instant::now();

    let now = OffsetDateTime::now_utc();
    let now_jst = now + Time_Duration::hours(9);
    let table_name = format!("db_{:04}{:02}{:02}_{:02}{:02}{:02}",
                             now_jst.year(), now_jst.month() as u8, now_jst.day(),
                             now_jst.hour(), now_jst.minute(), now_jst.second());

    let mut conn = pool.get_conn().map_err(LoadTestError::from)?;
    single_execute_query(&mut conn, &table_name).map_err(LoadTestError::from)?;

    let duration = start_time.elapsed();
    println!("事前クエリ実行完了。実行時間: {:?}", duration);

    Ok(table_name)
}

async fn monitor_progress(mut metrics_rx: mpsc::Receiver<MetricsUpdate>, total_batches: usize) {
    let mut last_count = 0;
    let mut last_update = Instant::now();

    while let Some(update) = metrics_rx.recv().await {
        let current_time = Instant::now();
        if current_time.duration_since(last_update) >= Duration::from_secs(1) {
            let records_per_second = (update.total_records - last_count) as f64;
            println!(
                "経過時間: {:.3}秒 ({} ms), 毎秒の処理レコード数: {:.0}, バッチ数: {} ({:.1}%), 総レコード数: {}    ",
                update.elapsed_time.as_secs_f64(),
                update.elapsed_time.as_millis(),
                records_per_second,
                update.batches_executed,
                (update.batches_executed as f64 / total_batches as f64 * 100.0),
                update.total_records,
            );
            io::stdout().flush().unwrap();

            last_count = update.total_records;
            last_update = current_time;
        }

        if update.batches_executed >= total_batches {
            println!(); // 最終行の改行
            break;
        }
    }
}

pub async fn run_load_test(config: Config) -> Result<()> {
    let pool = create_pool(&config.database_url)?;
    let table_name = execute_single_query(&pool).await?;
    let table_name = Arc::new(table_name);

    let total_records = config.total_batches * BATCH_SIZE;
    println!(
        "{}個の接続で{}バッチ（合計{}レコード）を実行中",
        config.connections,
        config.total_batches,
        total_records
    );

    let batches_executed = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    // メトリクス更新用のチャネルを作成
    let (metrics_tx, metrics_rx) = mpsc::channel(100);

    let start_time = Instant::now();
    let batches_executed_clone = Arc::clone(&batches_executed);

    // モニタリングタスク
    let monitoring_handle = tokio::spawn(monitor_progress(metrics_rx, config.total_batches));

    // メトリクス収集タスク
    let metrics_handle = {
        let metrics_tx = metrics_tx.clone();
        let batches_executed = Arc::clone(&batches_executed);
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(100));
            loop {
                interval.tick().await;
                let current_batches = batches_executed.load(Ordering::Relaxed);
                let update = MetricsUpdate {
                    batches_executed: current_batches,
                    total_records: current_batches * BATCH_SIZE,
                    elapsed_time: start_time.elapsed(),
                };

                if metrics_tx.send(update).await.is_err() {
                    break;
                }

                if current_batches >= config.total_batches {
                    break;
                }
            }
        })
    };

    // バッチの総数を接続数で割って、各接続が処理すべきバッチ数を計算
    let batches_per_connection = (config.total_batches + config.connections - 1) / config.connections;

    // バッチ実行タスク
    for i in 0..config.connections {
        let pool = pool.clone();
        let batches_executed = Arc::clone(&batches_executed);
        let table_name = Arc::clone(&table_name);
        let start_batch = i * batches_per_connection;
        let end_batch = std::cmp::min((i + 1) * batches_per_connection, config.total_batches);

        let handle: tokio::task::JoinHandle<Result<()>> = tokio::spawn(async move {
            let mut conn = pool.get_conn().map_err(LoadTestError::from)?;

            for _ in start_batch..end_batch {
                if batches_executed.fetch_add(1, Ordering::Relaxed) < config.total_batches {
                    execute_query(&mut conn, &table_name).map_err(LoadTestError::from)?;
                }
            }
            Ok(())
        });
        handles.push(handle);
    }

    // タスクの完了を待つ
    for handle in handles {
        match handle.await {
            Ok(result) => {
                if let Err(e) = result {
                    eprintln!("\nバッチ実行中にエラーが発生しました: {}", e);
                }
            }
            Err(e) => eprintln!("\nタスクの実行中にエラーが発生しました: {}", e),
        }
    }

    // メトリクスとモニタリングタスクを終了
    metrics_handle.abort();
    monitoring_handle.abort();

    let total_duration = start_time.elapsed();
    let total_batches_executed = batches_executed_clone.load(Ordering::Relaxed);
    let total_records_executed = total_batches_executed * BATCH_SIZE;
    let avg_records_per_second = total_records_executed as f64 / total_duration.as_secs_f64();

    println!("\nテスト完了");
    println!("テーブル名: {}", table_name);
    println!("合計実行時間: {:?}", total_duration);
    println!("実行されたバッチ数: {}", total_batches_executed);
    println!("実行されたレコード総数: {}", total_records_executed);
    println!("平均レコード数/秒: {:.2}", avg_records_per_second);

    Ok(())
}