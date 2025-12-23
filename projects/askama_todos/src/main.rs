use askama_todos::models::app::AppState;
use askama_todos::{init, routes};
use tokio::net::TcpListener;
use tracing;

#[tokio::main]
async fn main() {
	init::variables();
	init::logging();
	let config = init::config();

	let connection_pool = init::database_connection().await;
	let app_state = AppState { connection_pool };

	tracing::info!("Server is starting...");
	let listener = TcpListener::bind(config.get_addr())
		.await
		.expect("Failed to bind the listener");

	let app = routes::router(app_state);
	tracing::info!("Listening at {}", config.get_addr());

	axum::serve(listener, app)
		.await
		.expect("Failed to start the server");
}
