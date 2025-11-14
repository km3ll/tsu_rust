#![allow(unused)]
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
	name: String,
}

fn load_config() -> Result<AppConfig, ConfigError> {
	let settings = Config::builder()
		.add_source(File::with_name("config/config.yaml"))
		.build()?;
	settings.get::<AppConfig>("app")
}

fn main() {
	let config = load_config().unwrap();
	println!("{:?}", config)
}
