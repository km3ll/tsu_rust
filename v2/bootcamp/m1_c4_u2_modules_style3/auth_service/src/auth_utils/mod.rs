pub mod models;
pub fn login(creds: models::Credentials) {
    println!("authenticate...");
    crate::database::get_user();
}
pub fn logout() {
    println!("log user out...");
}