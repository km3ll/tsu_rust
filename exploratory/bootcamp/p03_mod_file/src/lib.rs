// pod: crate-level allow attribute
#![allow(dead_code, unused_variables)]

mod auth_utils;
mod database;

use auth_utils::models::Credentials;

pub fn authenticate(creds: Credentials) {
    if let database::Status::Connected = database::connect_to_db() {
        auth_utils::login(creds)
    }
}
