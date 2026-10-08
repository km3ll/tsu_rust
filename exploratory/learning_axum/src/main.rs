use axum::{
	Json, Router,
	http::StatusCode,
	routing::{get, post},
};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

#[derive(Deserialize)]
struct CreateUser {
	username: String,
}

#[derive(Serialize)]
struct User {
	id: u64,
	username: String,
}

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt().init();

	let trace_layer = TraceLayer::new_for_http()
		.make_span_with(DefaultMakeSpan::new().level(Level::INFO))
		.on_request(DefaultOnRequest::new().level(Level::INFO))
		.on_response(DefaultOnResponse::new().level(Level::INFO));

	// application with a route
	let app = Router::new()
		.route("/", get(root))
		.route("/users", post(create_user))
		.layer(trace_layer);

	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

	tracing::info!("listening on {}", listener.local_addr().unwrap());
	axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
	"Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
	let user = User {
		id: rand::rng().random_range::<u64, _>(1..=9999),
		username: payload.username,
	};

	(StatusCode::CREATED, Json(user))
}
