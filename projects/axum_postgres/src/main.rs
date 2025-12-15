use axum::Router;
use tokio::net::TcpListener;
use std::time::Duration;
use axum::routing::{delete, get, post};
use sqlx::postgres::PgPoolOptions;
use crate::db::{create_task, delete_task, get_tasks};

mod db;

#[tokio::main]
async fn main() {
	// Environment variables
	dotenvy::dotenv().expect("Unable to access .env file");
	let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("172.0.0.1:3000".to_owned());
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");

	// Database pool
	let db_pool = PgPoolOptions::new()
		.max_connections(64)
		.acquire_timeout(Duration::from_secs(5))
		.connect(&database_url)
		.await
		.expect("Cannot connect to the database");

	let listener = TcpListener::bind(server_address)
		.await
		.expect("Cannot create TCP listener");

	println!("Listening on: {}", listener.local_addr().unwrap());

	let app = Router::new()
		.route("/tasks", get(get_tasks))
		.route("/tasks", post(create_task))
		.route("/tasks/:task_id", delete(delete_task))
		.with_state(db_pool);

	axum::serve(listener, app)
		.await
		.expect("Cannot start serving the application");
}
