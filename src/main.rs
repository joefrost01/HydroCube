mod config;
mod db;
mod ingestion;
mod server;
mod aggregation;

use actix_web::web;
use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use r2d2::Pool;
use rustls::crypto::{self, CryptoProvider};
use crate::config::cli::Cli;
use crate::config::config::{AppConfig, FileFormat};
use crate::db::db_pool::DuckDBConnectionManager;
use crate::ingestion::directory_watcher::directory_watcher;
use crate::server::web_server;

#[actix_web::main]
async fn main() -> Result<()> {
    // Install the crypto provider for TLS.
    CryptoProvider::install_default(crypto::ring::default_provider()).unwrap();

    // Parse command-line arguments and load configuration.
    let cli = Cli::parse();
    let config_contents = fs::read_to_string(&cli.config)
        .with_context(|| format!("Failed to read config file {}", &cli.config))?;
    let config: AppConfig =
        serde_yaml::from_str(&config_contents).context("Failed to parse YAML config")?;

    // Wrap the config in Actix's web::Data (this internally uses an Arc for cheap cloning).
    let config_data = web::Data::new(config);

    // Set up the DuckDB connection pool.
    let connection_string = "/tmp/hydrocube.duckdb";
    let manager = DuckDBConnectionManager::new(connection_string.to_string());
    let pool: Pool<DuckDBConnectionManager> = Pool::new(manager)
        .expect("Failed to create DuckDB connection pool");

    // Spawn directory watchers for each dataset.
    for dataset in config_data.datasets.clone() {
        let pool_clone = pool.clone();

        // Here `dataset` is an owned DatasetConfig if your config_data.datasets is a Vec<DatasetConfig>
        // But let's still clone what we need so they outlive this for-loop.

        if let Some(dir) = dataset.directory.clone() {
            // Now `dir` is a String owned by us, not a borrowed reference.
            let dataset_cloned = dataset.clone();

            tokio::spawn(async move {
                // We have owned `dir` (String) and owned `dataset_cloned` (DatasetConfig).
                // No lifetime issues: they live within this async task.
                if let Err(e) = directory_watcher(&dir, pool_clone, dataset_cloned).await {
                    eprintln!("Directory watcher error: {:?}", e);
                }
            });
        }
    }


    // (Optional) Set up OAuth if enabled.
    if config_data.security.oauth.enabled {
        println!("OAuth is enabled for provider: {}", config_data.security.oauth.provider);
        // Insert your OAuth flow setup here.
    }

    // Start the server.
    web_server::run_server(pool, config_data).await
}