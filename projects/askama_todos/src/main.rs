use askama_todos::{init, routes};
use tokio::net::TcpListener;
use tracing;

#[tokio::main]
async fn main() {
	init::variables();
	init::logging();
	let config = init::config();

	init::database_connection().await;

	tracing::info!("Server is starting...");
	let listener = TcpListener::bind(config.get_addr())
		.await
		.expect("Failed to bind the listener");

	let app = routes::router();
	tracing::info!("Listening at {}", config.get_addr());

	axum::serve(listener, app)
		.await
		.expect("Failed to start the server");
}
