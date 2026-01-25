use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerCfg {
	pub host: String,
	pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
	pub server: ServerCfg,
}

impl AppConfig {
	pub fn from_file(base_name: &str) -> Result<Self, ConfigError> {
		let config = Config::builder()
			.add_source(File::with_name(base_name))
			.build()?;
		config.try_deserialize()
	}
}
