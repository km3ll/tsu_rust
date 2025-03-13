// crate-level allow attribute
#![allow(dead_code, unused)]

mod database {
	pub enum Status {
		Connected,
		Interrupted,
	}
	pub fn connect_to_database() -> Status {
		return Status::Connected;
	}
	pub fn get_user() {
		println!("get user from database...");
	}
}

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

/***
 * Local name bindings to a given path
 * - Bringing items into scope
 */
use auth_utils::models::Credentials;
use database::Status;

/***
> cargo modules structure

crate auth_service
├── mod auth_utils: pub(crate)
│   ├── fn login: pub
│   ├── fn logout: pub
│   └── mod models: pub
│       └── struct Credentials: pub
├── fn authenticate: pub
└── mod database: pub(crate)
    ├── enum Status: pub
    ├── fn connect_to_database: pub
    └── fn get_user: pub
 */
pub fn authenticate(creds: Credentials) {
	if let Status::Connected = database::connect_to_database() {
		auth_utils::login(creds);
	}
}