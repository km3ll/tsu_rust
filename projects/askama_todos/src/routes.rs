use crate::handlers::{auth::*, public::*, todos::*};
use axum::{Router, routing::get};
use tower_http::services::ServeDir;

pub fn router() -> Router {
	let server_dir = ServeDir::new("static");
	Router::new()
		.route("/", get(home_handler))
		.route("/create", get(create_todo_handler))
		.route("/todos", get(todos_handler))
		.route("/sign-up", get(sign_up_handler))
		.route("/log-in", get(log_in_handler))
		.nest_service("/static", server_dir)
}
