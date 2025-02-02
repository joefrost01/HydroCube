use clap::Parser;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the config file
    #[arg(long, default_value = "config.yaml")]
    pub config: String,
}
