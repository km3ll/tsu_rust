use super::helpers;
use crate::data::errors::DataError;
use crate::data::user::create_user;
use crate::handlers::errors::AppError;
use crate::models::app::AppState;
use crate::models::templates::{LoginTemplate, SignupTemplate};
use crate::models::user_form_model::AuthFormModel;
use askama::Template;
use axum::Form;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::response::{IntoResponse, Response};
use validator::Validate;

pub async fn get_sign_up_handler() -> Result<Response, AppError> {
	let page = SignupTemplate {
		email: "",
		email_error: "",
		password_error: "",
	}
	.render()?;
	Ok(Html(page).into_response())
}

pub async fn post_sign_up_handler(
	State(app_state): State<AppState>,
	Form(user_form): Form<AuthFormModel>,
) -> Result<Response, AppError> {
	match user_form.validate() {
		Ok(_) => {
			let result = create_user(
				&app_state.connection_pool,
				&user_form.email,
				&user_form.password,
			)
			.await;
			if let Err(err) = result {
				if let DataError::FailedQuery(e) = err {
					tracing::error!("Failed to sign up {}", e);
					return Ok(Redirect::to("/sign-up").into_response());
				} else {
					Err(err)?
				}
			}
			Ok(Redirect::to("/log-in").into_response())
		}
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
			.render()?;

			let response = Html(html_string).into_response();
			Ok((StatusCode::BAD_REQUEST, response).into_response())
		}
	}
}

pub async fn log_in_handler() -> Result<Response, AppError> {
	let page = LoginTemplate {}.render().unwrap();
	Ok(Html(page).into_response())
}
