#![allow(dead_code, unused)]

mod database;

mod auth_utils {
	pub mod models {
		pub struct Credentials {
			username: String,
			password: String,
		}
	}
	pub fn login(creds: models::Credentials) {
		println!("authenticate...");
		crate::database::get_user();
	}
	pub fn logout() {
		println!("log user out...");
	}
}

use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
	if let Status::Connected = database::connect_to_database() {
		auth_utils::login(creds);
	}
}