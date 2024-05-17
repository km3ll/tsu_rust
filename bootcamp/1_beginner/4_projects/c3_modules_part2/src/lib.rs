#![allow(unused)]

// Module without any sub-modules.
// Modules are not mappet to the file system, so because the content of the 
// database module is not defined in line Rust will look for a file calle database.rs
mod database;
mod auth_utils;

// Local name bindings
// The use declaration is bringing Credentials into scope. In the Rustlang book, the language
// tends to be "bringing items into scope".
use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}