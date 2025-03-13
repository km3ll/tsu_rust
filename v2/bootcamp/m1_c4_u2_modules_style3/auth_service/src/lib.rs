#![allow(dead_code, unused)]

mod database;
mod auth_utils;

use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
	if let Status::Connected = database::connect_to_database() {
		auth_utils::login(creds);
	}
}