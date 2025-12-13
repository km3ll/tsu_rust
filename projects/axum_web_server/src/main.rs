#![allow(dead_code)]
use axum::{
	Json, Router,
	extract::Path,
	http::StatusCode,
	response::{IntoResponse, Response},
	routing::get,
};
use serde_json::{Value, json};
use tokio::net::TcpListener;

#[derive(Debug)]
enum ApiError {
	NotFound,
	InvalidInput(String),
	InternalError,
}

impl IntoResponse for ApiError {
	fn into_response(self) -> Response {
		let (status, error_message) = match self {
			ApiError::NotFound => (StatusCode::NOT_FOUND, "Data not found".to_string()),
			ApiError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
			ApiError::InternalError => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Internal server error".to_string(),
			),
		};

		let body = Json(json!({
			"error": error_message
		}));

		(status, body).into_response()
	}
}

async fn health_check() -> impl IntoResponse {
	Json(json!({
		"status": "ok",
		"message": "Server is running"
	}))
}

async fn list_users() -> Result<Json<Value>, ApiError> {
	Err(ApiError::InternalError)
}

async fn get_user(Path(id): Path<u32>) -> Result<Json<Value>, ApiError> {
	if id > 100 {
		return Err(ApiError::NotFound);
	}
	Ok(Json(json!({
		"id": id,
		"user": "Zoro"
	})))
}

fn create_app() -> Router {
	Router::new()
		.route("/health", get(health_check))
		.route("/users", get(list_users))
		.route("/users/{id}", get(get_user))
}

#[tokio::main]
async fn main() {
	let app = create_app();

	let listener = TcpListener::bind("localhost:3000")
		.await
		.expect("Failed to bind tcp listener");
	println!("Server running on http://localhost:3000");

	axum::serve(listener, app)
		.await
		.expect("Failed to start server");
}

#[cfg(test)]
mod tests {
	use super::*;
	use axum::{body::Body, http::Request};
	use tower::ServiceExt;

	#[tokio::test]
	async fn test_health_check() {
		let app = create_app();

		let request = Request::builder()
			.method("GET")
			.uri("/health")
			.body(Body::empty())
			.unwrap();

		let response = app.oneshot(request).await.unwrap();
		assert_eq!(response.status(), StatusCode::OK);
	}
}
