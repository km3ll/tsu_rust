// pod: crate-level allow attribute
#![allow(dead_code, unused_variables)]

mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_db() -> Status {
        Status::Connected
    }

    pub fn get_user() {
        // get user from database...
    }
}

mod auth_utils {
    pub fn login(creds: models::Credentials) {
        // authenticate...
        crate::database::get_user();
    }
    pub fn logout() {
        // log user out...
    }

    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}

use auth_utils::models::Credentials;

pub fn authenticate(creds: Credentials) {
    if let database::Status::Connected = database::connect_to_db() {
        auth_utils::login(creds)
    }
}
