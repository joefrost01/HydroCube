use anyhow::Result;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use r2d2::Pool;
use std::path::Path;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;
use crate::config::config::{DatasetConfig, FileFormat};
use crate::db::db_pool::DuckDBConnectionManager;
use crate::ingestion::handlers::{ingest_csv, ingest_json, ingest_parquet};

pub async fn directory_watcher(
    watch_path: &str,
    pool: Pool<DuckDBConnectionManager>,
    dataset: DatasetConfig,
) -> Result<()> {
    // Create an asynchronous channel to receive events.
    let (tx, mut rx) = mpsc::channel::<Event>(100);

    // Create a file system watcher.
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    // Inside this callback we’re in a synchronous context,
                    // so we use blocking_send.
                    if let Err(e) = tx.blocking_send(event) {
                        eprintln!("Error sending event: {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Watch error: {:?}", e);
                }
            }
        },
        notify::Config::default().with_poll_interval(Duration::from_secs(2)),
    )?;

    // Start watching the directory (recursively).
    watcher.watch(Path::new(watch_path), RecursiveMode::Recursive)?;

    println!("Started watching directory: {}", watch_path);

    // Process file system events as they come in.
    while let Some(event) = rx.recv().await {
        println!("Received event in {}: {:?}", dataset.name, event);

        // For each event, spawn a blocking task that performs ingestion.
        // Clone the pool and dataset config so the task can run independently.
        let pool_clone = pool.clone();
        let dataset_clone = dataset.clone();

        task::spawn_blocking(move || {
            // Get a connection from the pool.
            let conn = match pool_clone.get() {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("Error getting connection from pool: {:?}", e);
                    return;
                }
            };

            // Based on the dataset's file format, trigger the appropriate ingestion.
            // (Assume these functions—ingest_csv, ingest_parquet, ingest_json—are defined in your ingest module.)
            match dataset_clone.format {
                FileFormat::Csv => {
                    if let Err(e) = ingest_csv(&conn, &dataset_clone) {
                        eprintln!("Error ingesting CSV for {}: {:?}", dataset_clone.name, e);
                    } else {
                        println!("Successfully ingested CSV dataset {}", dataset_clone.name);
                    }
                }
                FileFormat::Parquet => {
                    if let Err(e) = ingest_parquet(&conn, &dataset_clone) {
                        eprintln!("Error ingesting Parquet for {}: {:?}", dataset_clone.name, e);
                    } else {
                        println!("Successfully ingested Parquet dataset {}", dataset_clone.name);
                    }
                }
                FileFormat::Json => {
                    if let Err(e) = ingest_json(&conn, &dataset_clone) {
                        eprintln!("Error ingesting JSON for {}: {:?}", dataset_clone.name, e);
                    } else {
                        println!("Successfully ingested JSON dataset {}", dataset_clone.name);
                    }
                }
                FileFormat::Kafka => {
                    todo!("Implement Kafka ingestion here");
                }
            }
        });
    }

    Ok(())
}
