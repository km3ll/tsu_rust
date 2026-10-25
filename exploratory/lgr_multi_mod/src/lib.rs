#![allow(dead_code, unused_variables)]
use crate::auth_utils::models::Credentials;
use crate::database::Status;
use rand::prelude::*;

pub mod auth_utils;
pub mod database;

pub fn authenticate(creds: Credentials) {
    println!(" > func: authenticate()");

    let timeout = thread_rng().gen_range(100..500);
    println!(" > func: timeout: {timeout}");

    if let Status::Connected = database::connect_to_database() {
        println!(" > func: status: connected");
        auth_utils::login(creds);
    }
}
