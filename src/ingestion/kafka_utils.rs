// ingestion/kafka_utils.rs (for example)
use r2d2::Pool;
use crate::db::db_pool::DuckDBConnectionManager;
use crate::config::config::{KafkaTopicConfig};

/// Creates the table if it doesn't exist
pub fn create_table_if_not_exists(
    pool: &Pool<DuckDBConnectionManager>,
    topic_config: &KafkaTopicConfig
) -> anyhow::Result<()> {
    let mut conn = pool.get()?;

    // Build a CREATE TABLE statement from the config's schema
    let columns_ddl: String = topic_config
        .schema
        .iter()
        .map(|field| format!("{} {}", field.column, field.field_type))
        .collect::<Vec<_>>()
        .join(", ");

    let create_sql = format!(
        "CREATE TABLE IF NOT EXISTS {} ({});",
        topic_config.table_name, columns_ddl
    );

    conn.execute(&create_sql, [])?;
    Ok(())
}
