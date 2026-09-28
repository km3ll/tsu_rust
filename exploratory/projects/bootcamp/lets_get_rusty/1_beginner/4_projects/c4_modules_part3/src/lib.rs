use rand::prelude::*;

// The crate module has two sub-modules 
mod database;
mod auth_utils;

// We are bringing Credentials and Status into scope
// 'use' statements are also used to re-exporting (using  the 'pub' keyword)
pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    let timeout = thread_rng().gen_range(100..500);
    println!("The timeout is {timeout}");

    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
        println!("[Authenticated!]");
    }
}