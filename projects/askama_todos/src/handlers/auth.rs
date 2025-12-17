use crate::models::templates::{LoginTemplate, SignupTemplate};
use askama::Template;
use axum::response::Html;
use axum::response::{IntoResponse, Response};

pub async fn sign_up_handler() -> Response {
	let page = SignupTemplate {}.render().unwrap();
	Html(page).into_response()
}
pub async fn log_in_handler() -> Response {
	let page = LoginTemplate {}.render().unwrap();
	Html(page).into_response()
}
