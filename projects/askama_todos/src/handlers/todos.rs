use crate::models::templates::{CreateTemplate, TodosTemplate};
use askama::Template;
use axum::response::Html;
use axum::response::{IntoResponse, Response};

pub async fn create_todo_handler() -> Response {
	let page = CreateTemplate {}.render().unwrap();
	Html(page).into_response()
}
pub async fn todos_handler() -> Response {
	let page = TodosTemplate {}.render().unwrap();
	Html(page).into_response()
}
