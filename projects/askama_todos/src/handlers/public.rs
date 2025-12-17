use crate::models::templates::HomeTemplate;
use askama::Template;
use axum::response::Html;
use axum::response::{IntoResponse, Response};

pub async fn home_handler() -> Response {
	let page = HomeTemplate {}.render().unwrap();
	Html(page).into_response()
}
