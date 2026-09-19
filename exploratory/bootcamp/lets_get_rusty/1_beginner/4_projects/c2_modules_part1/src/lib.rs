// Crate-level allow attribute
#![allow(dead_code, unused_variables)]

// Modules do not map to the file system
// In this file we have multiple levels of abstraction and mixing concerns

mod database {

    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        return Status::Connected;
    }

    pub fn get_user() {
        // get user form database...
    }

}

mod auth_utils {

    // Relative path: models::Credentials
    pub fn login(creds: models::Credentials) {
        // authenticate...
        // Absolute path: crate::database::get_user();
        crate::database::get_user();
    }
    
    fn logout() {
        // log user out...
    }

    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }

}

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