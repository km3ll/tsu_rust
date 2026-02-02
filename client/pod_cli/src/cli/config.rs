use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CliConfig {
	pub server: ServerConfig,
}

impl CliConfig {
	pub fn new(server: ServerConfig) -> Self {
		CliConfig { server }
	}
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
	pub host: String,
	pub port: u16,
}

impl ServerConfig {
	pub fn new(host: String, port: u16) -> Self {
		ServerConfig { host, port }
	}
}

impl CliConfig {
	pub fn from_file(base_name: &str) -> Self {
		Config::builder()
			.add_source(File::with_name(base_name))
			.build()
			.expect(format!("Failed to load file: {base_name}").as_str())
			.try_deserialize()
			.expect(format!("Failed to deserialize file: {base_name}").as_str())
	}
}
