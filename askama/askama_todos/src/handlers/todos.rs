use crate::handlers::errors::AppError;
use crate::models::templates::{CreateTemplate, TodosTemplate};
use askama::Template;
use axum::response::Html;
use axum::response::{IntoResponse, Response};

pub async fn create_todo_handler() -> Result<Response, AppError> {
	let page = CreateTemplate {}.render().unwrap();
	Ok(Html(page).into_response())
}
pub async fn todos_handler() -> Result<Response, AppError> {
	let page = TodosTemplate {}.render().unwrap();
	Ok(Html(page).into_response())
}
