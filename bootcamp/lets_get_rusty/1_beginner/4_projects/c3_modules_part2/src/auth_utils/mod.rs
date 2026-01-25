// Relative path: models::Credentials
pub fn login(creds: models::Credentials) {
    // authenticate...
    // Absolute path: crate::database::get_user();
    crate::database::get_user();
}

fn logout() {
    // log user out...
}

pub mod models;