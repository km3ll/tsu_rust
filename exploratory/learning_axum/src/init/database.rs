use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use std::{str::FromStr, time::Duration};

pub async fn connection() -> PgPool {
	tracing::debug!("Setting up database connection");
	let db_url = dotenvy::var("DATABASE_URL").expect("Failed to load database URL from env");

	let options = PgConnectOptions::from_str(&db_url)
		.expect("Failed to parse database url")
		.disable_statement_logging();

	let pg_pool = PgPoolOptions::new()
		.acquire_timeout(Duration::from_secs(5))
		.max_connections(5)
		.connect_with(options)
		.await
		.expect("Failed to connect to the database");

	tracing::debug!("Successfully connected");

	sqlx::migrate!()
		.run(&pg_pool)
		.await
		.expect("Failed to run migrations");
	tracing::debug!("Successfully migrated");

	pg_pool
}
