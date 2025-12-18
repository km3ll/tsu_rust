use config::{Config, ConfigError, File};
use serde::Deserialize;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn get_addr(&self) -> String {
        format!("{}:{}", &self.host, &self.port)
    }
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn from_file(base_name: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(base_name))
            .build()?;
        config.try_deserialize()
    }
}

pub fn logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    let subscriber = FmtSubscriber::builder()
        //.with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up logging");
}

pub fn variables() {
    dotenvy::dotenv().expect("Failed to access .env file");
}

pub fn config() -> ServerConfig {
    AppConfig::from_file("config/config.yaml")
        .expect("Failed to load configuration")
        .server
}
