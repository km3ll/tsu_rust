use crate::models::templates::{LoginTemplate, SignupTemplate};
use crate::models::user_form_model::UserFormModel;
use askama::Template;
use axum::Form;
use axum::response::{Html, Redirect};
use axum::response::{IntoResponse, Response};
use tracing::info;

pub async fn get_sign_up_handler() -> Response {
	let page = SignupTemplate {}.render().unwrap();
	Html(page).into_response()
}

pub async fn post_sign_up_handler(Form(user_form): Form<UserFormModel>) -> Response {
	info!("Email is '{}'", user_form.email);
	Redirect::to("/").into_response()
}

pub async fn log_in_handler() -> Response {
	let page = LoginTemplate {}.render().unwrap();
	Html(page).into_response()
}
