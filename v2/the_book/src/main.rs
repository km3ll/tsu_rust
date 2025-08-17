#![allow(unused)]

// pod: attach file to main
mod config;
mod c01;

fn main() {
	// Load configuration
	let cfg = config::AppConfig::from_file()
		.expect("Failed to load config file");
	println!("Server will run on {}:{}", cfg.server.host, cfg.server.port);
}
