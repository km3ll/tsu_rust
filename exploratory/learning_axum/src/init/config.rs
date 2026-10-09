use config::{ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
	pub host: String,
	pub port: u16,
}

impl ServerConfig {
	pub fn get_address(&self) -> String {
		format!("{}:{}", self.host, self.port)
	}
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
	pub server: ServerConfig,
}

impl AppConfig {
	pub fn from_file(path: &str) -> Result<Self, ConfigError> {
		let config = config::Config::builder()
			.add_source(File::with_name(path))
			.build()?;
		config.try_deserialize()
	}
}

pub fn load() -> AppConfig {
	AppConfig::from_file("config/config.yaml").expect("Failed to load config file")
}
