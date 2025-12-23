use crate::handlers::{auth::*, public::*, todos::*};
use crate::models::app::AppState;
use axum::body::Body;
use axum::http::{Request, Response};
use axum::{Router, routing::get};
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{Span, error, info};

pub fn router(app_state: AppState) -> Router {
	let server_dir = ServeDir::new("static");

	let trace_layer = TraceLayer::new_for_http()
		.make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
		.on_request(on_request)
		.on_response(on_response)
		.on_failure(on_failure);

	Router::new()
		.route("/", get(home_handler))
		.route("/create", get(create_todo_handler))
		.route("/todos", get(todos_handler))
		.route(
			"/sign-up",
			get(get_sign_up_handler).post(post_sign_up_handler),
		)
		.route("/log-in", get(log_in_handler))
		.nest_service("/static", server_dir)
		.with_state(app_state)
		.layer(trace_layer)
}

fn on_request(request: &Request<Body>, _: &Span) {
	info!(
		"-> equest started: method {} path {}",
		request.method(),
		request.uri().path()
	)
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
	info!(
		"<- Response generated: status {} in {:?}",
		response.status(),
		latency
	)
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
	error!("-x- Response failed: {:?} after {:?}", error, latency)
}
