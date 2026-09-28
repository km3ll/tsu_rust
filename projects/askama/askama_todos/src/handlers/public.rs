use crate::handlers::errors::AppError;
use crate::models::templates::HomeTemplate;
use askama::Template;
use axum::response::Html;
use axum::response::{IntoResponse, Response};

pub async fn home_handler() -> Result<Response, AppError> {
	let page = HomeTemplate {}.render().unwrap();
	Ok(Html(page).into_response())
}
