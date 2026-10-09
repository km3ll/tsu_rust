use axum::{Router, routing::get};
use learning_axum::{http::handlers::find_all_zettels, init, init::app::AppState};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
	init::env::load();
	init::logging::start();
	let config = init::config::load();
	let shared_state = Arc::new(app_state().await);

	let app = Router::new()
		.route("/", get(|| async { "Hello, World!" }))
		.route("/zettels", get(find_all_zettels))
		.with_state(shared_state)
		.layer(TraceLayer::new_for_http());

	let listener = TcpListener::bind(config.server.get_address())
		.await
		.unwrap();

	tracing::info!("Listening on {}", listener.local_addr().unwrap());
	axum::serve(listener, app).await.unwrap();
}

async fn app_state() -> AppState {
	let pg_pool = init::database::connection().await;
	AppState::new(pg_pool)
}
