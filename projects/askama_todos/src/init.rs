use config::{Config, ConfigError, File};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use std::{str::FromStr, time::Duration};
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

pub async fn database_connection() -> PgPool {
	tracing::debug!("Setting up database connection");
	let db_url = dotenvy::var("DATABASE_URL").expect("Failed to get database URL from env");

	let options = PgConnectOptions::from_str(&db_url)
		.expect("Failed to parse database URL")
		.disable_statement_logging();

	let pg_pool = PgPoolOptions::new()
		.acquire_timeout(Duration::from_secs(5))
		.connect_with(options)
		.await
		.expect("Failed to connect to the database");

	tracing::debug!("Successfully connected");

	sqlx::migrate!()
		.run(&pg_pool)
		.await
		.expect("Failed to migrate");

	tracing::debug!("Successfully migrated");

	pg_pool
}
