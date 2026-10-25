use models::Credentials;

pub mod models;

pub fn login(creds: Credentials) {
    println!(" > func: login()");
    crate::database::get_user();
}

pub fn logout() {
    println!(" > func: logout()");
}
