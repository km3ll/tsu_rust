#![allow(unused)]

pub mod models;

// Relative path: models::Credentials
pub fn login(creds: models::Credentials) {
    // authenticate...
    // Absolute path: crate::database::get_user();
}

fn logout() {
    // log user out...
}