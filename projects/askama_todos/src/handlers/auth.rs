use super::helpers;
use crate::models::templates::{LoginTemplate, SignupTemplate};
use crate::models::user_form_model::AuthFormModel;
use askama::Template;
use axum::Form;
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::response::{IntoResponse, Response};
use validator::Validate;
pub async fn get_sign_up_handler() -> Response {
	let page = SignupTemplate {
		email: "",
		email_error: "",
		password_error: "",
	}
	.render()
	.unwrap();
	Html(page).into_response()
}

pub async fn post_sign_up_handler(Form(user_form): Form<AuthFormModel>) -> Response {
	match user_form.validate() {
		Ok(_) => Redirect::to("/").into_response(),
		Err(err) => {
			let errors = err.to_string();
			let mut email_error = String::new();
			let mut password_error = String::new();

			helpers::extract_error(&errors, |field, message| {
				if field == "email" {
					email_error = message
				} else if field == "password" {
					password_error = message
				}
			});

			let html_string = SignupTemplate {
				email: &user_form.email,
				email_error: &email_error,
				password_error: &password_error,
			}
			.render()
			.unwrap();

			let response = Html(html_string).into_response();
			(StatusCode::BAD_REQUEST, response).into_response()
		}
	}
}

pub async fn log_in_handler() -> Response {
	let page = LoginTemplate {}.render().unwrap();
	Html(page).into_response()
}
