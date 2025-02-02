use crate::config::config::{DatasetConfig, FileFormat};
use anyhow::Result;
use duckdb::Connection;

pub fn ingest_dataset(conn: &Connection, dataset: &DatasetConfig) -> Result<()> {
    match dataset.format {
        FileFormat::Csv => ingest_csv(conn, dataset),
        FileFormat::Parquet => ingest_parquet(conn, dataset),
        FileFormat::Json => ingest_json(conn, dataset), // Add this line
        FileFormat::Kafka => todo!("Implement Kafka ingestion here"),
    }
}

pub fn ingest_csv(conn: &Connection, dataset: &DatasetConfig) -> Result<()> {
    let directory = dataset.directory.as_deref().unwrap_or(".");
    let pattern = dataset.pattern.as_deref().unwrap_or("*.csv");
    let sql = format!(
        r#"
        CREATE TABLE IF NOT EXISTS "{table_name}" AS
        SELECT * FROM read_csv_auto('{directory}/{pattern}');
        "#,
        table_name = dataset.name,
    );

    conn.execute(&sql, [])?;
    Ok(())
}

pub fn ingest_parquet(conn: &Connection, dataset: &DatasetConfig) -> Result<()> {
    let directory = dataset.directory.as_deref().unwrap_or(".");
    let pattern = dataset.pattern.as_deref().unwrap_or("*.parquet");
    let sql = format!(
        r#"
        CREATE TABLE IF NOT EXISTS "{table_name}" AS
        SELECT * FROM parquet_scan('{directory}/{pattern}');
        "#,
        table_name = dataset.name,
    );

    conn.execute(&sql, [])?;
    Ok(())
}

pub fn ingest_json(conn: &Connection, dataset: &DatasetConfig) -> Result<()> {
    let directory = dataset.directory.as_deref().unwrap_or(".");
    let pattern = dataset.pattern.as_deref().unwrap_or("*.json");
    conn.execute("INSTALL httpfs; LOAD httpfs;", [])?;
    let sql = format!(
        "CREATE TABLE \"{table_name}\" AS
         SELECT * FROM read_ndjson_auto('{directory}/{pattern}')",
        table_name = dataset.name,
    );
    conn.execute(&sql, [])?;
    Ok(())
}
