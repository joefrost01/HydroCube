use crate::config::config::{DatasetConfig, FileFormat};
use anyhow::Result;
use duckdb::Connection;

pub fn ingest_dataset(conn: &Connection, dataset: &DatasetConfig) -> Result<()> {
    match dataset.format {
        FileFormat::Csv => ingest_csv(conn, dataset),
        FileFormat::Parquet => ingest_parquet(conn, dataset),
        FileFormat::Json => ingest_json(conn, dataset),
    }
}

pub fn ingest_csv(conn: &Connection, dataset: &DatasetConfig) -> Result<()> {
    let sql = format!(
        r#"
        CREATE TABLE IF NOT EXISTS "{table_name}" AS
        SELECT * FROM read_csv_auto('{directory}/{pattern}');
        "#,
        table_name = dataset.name,
        directory = dataset.directory,
        pattern = dataset.pattern
    );

    conn.execute(&sql, [])?;
    Ok(())
}

pub fn ingest_parquet(conn: &Connection, dataset: &DatasetConfig) -> Result<()> {
    let sql = format!(
        r#"
        CREATE TABLE IF NOT EXISTS "{table_name}" AS
        SELECT * FROM parquet_scan('{directory}/{pattern}');
        "#,
        table_name = dataset.name,
        directory = dataset.directory,
        pattern = dataset.pattern
    );

    conn.execute(&sql, [])?;
    Ok(())
}

pub fn ingest_json(conn: &Connection, dataset: &DatasetConfig) -> Result<()> {
    // There's no built-in "read_json_auto", but you can do something similar:
    // e.g., using the JSON extension or the read_ndjson_auto function in newer DuckDB versions
    // For example:
    /*
    conn.execute("INSTALL httpfs; LOAD httpfs;", [])?;
    let sql = format!(
        "CREATE TABLE \"{table_name}\" AS
         SELECT * FROM read_ndjson_auto('{directory}/{pattern}')",
        ...
    );
    */
    // For now, stub it out:
    println!("JSON ingestion not implemented yet!");
    Ok(())
}
