pub fn load() {
	dotenvy::dotenv().expect("Failed to load .env file");
}
