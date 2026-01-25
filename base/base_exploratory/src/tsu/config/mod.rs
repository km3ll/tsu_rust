use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
	pub host: String,
	pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
	pub server: ServerConfig,
}

impl AppConfig {
	pub fn from_file() -> Result<Self, config::ConfigError> {
		let settings: Config = Config::builder()
			.add_source(File::with_name("config/application"))
			.build()?;
		settings.try_deserialize()
	}
}
