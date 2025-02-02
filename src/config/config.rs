use rustls::quic::Tag;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub datasets: Vec<DatasetConfig>,
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatasetConfig {
    pub name: String,

    // These only apply for file-based formats, so make them optional.
    #[serde(default)]
    pub directory: Option<String>,
    #[serde(default)]
    pub pattern: Option<String>,

    pub format: FileFormat,

    // If the format is Kafka, this field will hold Kafka-specific settings.
    #[serde(default)]
    pub kafka: Option<KafkaTopicConfig>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FileFormat {
    Csv,
    Parquet,
    Json,
    Kafka, // New variant for Kafka-based ingestion
}

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaTopicConfig {
    pub brokers: String,
    pub group_id: String,
    pub topic: String,
    pub schema: Vec<SchemaField>,
    pub table_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SchemaField {
    /// The name of the column in DuckDB
    pub column: String,

    /// The DuckDB type for this column (e.g. VARCHAR, INTEGER, DOUBLE)
    pub field_type: String,

    /// A JSON path expression or key pointing to the field in the Kafka message
    pub json_path: String,
}


// ------------------------------------------------------
// Security-related structs (unchanged, except for minor
// formatting or comments).
// ------------------------------------------------------

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub oauth: OAuthConfig,
    pub https: HttpsConfig,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        SecurityConfig {
            oauth: OAuthConfig::default(),
            https: HttpsConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct OAuthConfig {
    pub enabled: bool,
    pub provider: String,
    pub client_id: String,
    pub client_secret: String,
    /// The OAuth authorization endpoint URL.
    pub auth_url: String,
    /// The OAuth token endpoint URL.
    pub token_url: String,
    /// The redirect URL for OAuth callbacks.
    pub redirect_url: String,
    /// Optional scopes for OAuth.
    #[serde(default)]
    pub scopes: Vec<String>,
}

impl Default for OAuthConfig {
    fn default() -> Self {
        OAuthConfig {
            enabled: false,
            provider: "".into(),
            client_id: "".into(),
            client_secret: "".into(),
            auth_url: "".into(),
            token_url: "".into(),
            redirect_url: "".into(),
            scopes: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpsConfig {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
}

impl Default for HttpsConfig {
    fn default() -> Self {
        HttpsConfig {
            enabled: false,
            cert_path: "cert.pem".into(),
            key_path: "key.pem".into(),
        }
    }
}
